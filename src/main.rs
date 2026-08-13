use clap::Parser;
use csv::Writer;
use reqwest::Error;
use serde_yaml::{Value, from_str};
use std::{
    collections::BTreeMap,
    fs::{self, File, remove_file},
    io::{self, copy},
    path::{Path, PathBuf},
};
use tabld::{
    model::{CLASS, Graph, IndexedMemoryGraph, Subject},
    rdfxml,
};

#[derive(Parser, Debug)]
#[command(name = "manuscript_data", version, about, long_about = None)]
struct Cli {
    #[arg(short = 'd', long = "download")]
    download: bool,
    #[arg(short = 'l', long = "lazy")]
    lazy: bool,
    #[arg(short = 'o', long = "download-only")]
    download_only: bool,
    #[arg(short = 't', long = "test-length")]
    test_length: Option<u16>,
}

// Access a resource by url and read its contents to a string
fn read_to_string(url: String) -> Result<String, Error> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    Ok(body)
}

// Access a resource by url and download it to a destination path
fn download(url: String, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    let mut dest = File::create(destination)?;
    let content = response.bytes()?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}

// Get IDs and PURLs of active OBO ontologies (minus COB and BFO)
fn get_ontology_purls(
    yaml: String,
) -> Result<BTreeMap<String, BTreeMap<String, String>>, Box<dyn std::error::Error>> {
    let mut ont_info: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let registry: Value = from_str(&yaml)?;
    if let Some(x) = registry.get("ontologies") {
        if let Some(ont_list) = x.as_sequence() {
            for o in ont_list {
                let o_id = match o.get("id") {
                    Some(id) => match id.as_str() {
                        Some(id) => id.to_string(),
                        None => continue,
                    },
                    None => continue,
                };
                let o_purl = match o.get("ontology_purl") {
                    Some(purl) => match purl.as_str() {
                        Some(purl) => purl.to_string(),
                        None => format!("http://purl.obolibrary.org/obo/{o_id}.owl").to_string(),
                    },
                    None => format!("http://purl.obolibrary.org/obo/{o_id}.owl").to_string(),
                };
                let o_activity = match o.get("activity_status") {
                    Some(activity_status) => match activity_status.as_str() {
                        Some(activity_status) => activity_status.to_string(),
                        None => continue,
                    },
                    None => continue,
                };
                if o_id == String::from("cob") || o_id == String::from("bfo") {
                    continue;
                }
                let mut ont_map: BTreeMap<String, String> = BTreeMap::new();
                ont_map.insert("id".to_string(), o_id.clone());
                ont_map.insert("purl".to_string(), o_purl);
                ont_map.insert("activity_status".to_string(), o_activity);
                ont_info.insert(o_id.clone(), ont_map);
            }
        }
    }
    Ok(ont_info)
}

// Download ontologies
fn download_obo_onts(
    cache_dir: &str,
    unparseable_cache_dir: &str,
    summary_path: &str,
    lazy: bool,
    test_length: Option<u16>,
) {
    let registry = String::from(
        "https://raw.githubusercontent.com/OBOFoundry/OBOFoundry.github.io/master/registry/ontologies.yml",
    );
    let yaml = read_to_string(String::from(registry)).expect("Couldn't read YAML");
    let ont_info = get_ontology_purls(yaml).expect("Couldn't parse YAML");
    let mut downloads = 0;
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(summary_path)
        .unwrap();
    wtr.write_record([
        "Ontology ID",
        "Ontology PURL",
        "Activity Status",
        "Download Status",
    ])
    .unwrap();
    for id in ont_info.keys() {
        match test_length {
            Some(int) => {
                if downloads >= int {
                    break;
                }
            }
            None => (),
        }
        let ont = ont_info.get(id).unwrap();
        let filename = format!("{id}.owl");
        let path = format!("{cache_dir}/{filename}");
        let path = Path::new(&path);
        let rdfxml_error_path = format!("{unparseable_cache_dir}/{filename}");
        let rdfxml_error_path = Path::new(&rdfxml_error_path);
        if lazy {
            if path.exists() || rdfxml_error_path.exists() {
                eprintln!("Already downloaded {filename}");
                downloads += 1;
                wtr.write_record([
                    ont.get("id").unwrap(),
                    ont.get("purl").unwrap(),
                    ont.get("activity_status").unwrap(),
                    "Downloaded",
                ])
                .unwrap();
                continue;
            }
        }
        let purl = String::from(ont.get("purl").unwrap());
        let mut dl_status = "Not downloaded";
        match ont.get("activity_status") {
            Some(val) => {
                if val == "active" {
                    match download(purl, path) {
                        Ok(_) => {
                            eprintln!("Downloaded {filename}");
                            dl_status = "Downloaded";
                            downloads += 1;
                        }
                        Err(_) => {
                            eprintln!("Couldn't download {filename}");
                            match remove_file(path) {
                                Ok(_) => (),
                                Err(_) => (),
                            }
                        }
                    }
                    wtr.write_record([
                        ont.get("id").unwrap(),
                        ont.get("purl").unwrap(),
                        ont.get("activity_status").unwrap(),
                        dl_status,
                    ])
                    .unwrap();
                } else {
                    wtr.write_record([
                        ont.get("id").unwrap(),
                        ont.get("purl").unwrap(),
                        ont.get("activity_status").unwrap(),
                        dl_status,
                    ])
                    .unwrap();
                }
            }
            None => (),
        }
    }
}

// Iterate over an ontology file, outputting a CSV line for each class in the ontology
fn print_terms(
    ont_path: PathBuf,
    wtr: &mut Writer<File>,
    cob_subjects: &Vec<&Subject>,
) -> Result<(), String> {
    let ont = ont_path.file_prefix().unwrap().display();
    let ont_string = format!("{ont}");
    eprintln!("Gathering classes in {ont_string}");
    let rdfxml_input = match std::fs::read_to_string(ont_path) {
        Ok(string) => string,
        Err(err) => {
            return Err(format!("Could not read {ont_string}: {err:?}"));
        }
    };
    let graph = rdfxml::read(&rdfxml_input).expect("Read from string");
    // todo: should move files to unparseable/ automatically when tabld cannot read them
    // but tabld currently panics when it can't read the rdfxml in a file
    // so that change is pending different error handling in tabld
    let graph: IndexedMemoryGraph = graph.into();
    for subject in graph.subjects() {
        let mut in_base = "";
        if !subject.owl_types().contains(CLASS) {
            continue;
        }
        if subject.deprecated() {
            continue;
        }
        if subject.name() == "http://www.geneontology.org/formats/oboInOwl#ObsoleteClass"
            || subject.name() == "http://www.w3.org/2002/07/owl#Thing"
        {
            continue;
        }
        if subject
            .name()
            .to_lowercase()
            .contains(&ont_string.to_lowercase())
        {
            in_base = "T";
        }
        let mut term_ancestors = graph.ancestors(&subject.name());
        let name = subject.name();
        term_ancestors.insert(&name);
        let mut found: bool = false;
        for cob_subject in cob_subjects.iter() {
            if term_ancestors.contains(&cob_subject.name()) {
                wtr.write_record([
                    &subject.name(),
                    &subject.label().replace("\n", "").replace("\t", ""),
                    &cob_subject.name(),
                    &cob_subject.label(),
                    &ont_string,
                    in_base,
                    "",
                ])
                .unwrap();
                found = true;
                break;
            }
        }
        if !found {
            let mut top_ns_ancestor = "";
            if in_base == "T" {
                let mut anc_vec: Vec<&String> = term_ancestors.into_iter().collect();
                anc_vec.sort_by_key(|s| graph.ancestors(s).len());
                anc_vec.reverse();
                for ancestor in anc_vec.iter() {
                    if ancestor.to_lowercase().contains(&ont_string.to_lowercase()) {
                        top_ns_ancestor = ancestor;
                    } else {
                        break;
                    }
                }
            }
            wtr.write_record([
                &subject.name(),
                &subject.label().replace("\n", "").replace("\t", ""),
                "",
                "",
                &ont_string,
                in_base,
                top_ns_ancestor,
            ])
            .unwrap();
        }
    }
    Ok(())
}

fn generate_class_tsv(cob_path: &str, output_path: &str) {
    let rdfxml_input = std::fs::read_to_string(cob_path).expect("Read from file");
    let graph = rdfxml::read(&rdfxml_input).expect("Read from string");
    let cob_graph: IndexedMemoryGraph = graph.into();
    let mut cob_subjects: Vec<&Subject> = cob_graph.subjects().into_iter().collect();
    cob_subjects.sort_by_key(|s| cob_graph.ancestors(&s.name()).len());
    cob_subjects.reverse();

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(output_path)
        .unwrap();
    wtr.write_record([
        "Term IRI",
        "Term Label",
        "Ancestor IRI",
        "Ancestor Label",
        "Source",
        "In Base?",
        "Namespace Root",
    ])
    .unwrap();

    let mut entries = fs::read_dir("cache/")
        .expect("Could not read directory")
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .expect("Can't convert to path");
    entries.sort();

    for e in entries {
        if e.as_os_str() == cob_path {
            continue;
        }
        print_terms(e, &mut wtr, &cob_subjects).expect("couldn't print terms");
    }
}

fn main() {
    let cache_dir = "cache";
    let unparseable_cache_dir = "unparseable";
    let results_dir = "results";
    if !Path::new(cache_dir).exists() {
        fs::create_dir("cache").expect("Failed to create cache");
        eprintln!("Created directory: cache/")
    }
    if !Path::new(unparseable_cache_dir).exists() {
        fs::create_dir("unparseable").expect("Failed to create unparseable file dir");
        eprintln!("Created directory: unparseable/")
    }
    if !Path::new(results_dir).exists() {
        fs::create_dir("results").expect("Failed to create results dir");
        eprintln!("Created directory: results/")
    }
    let summary_path = format!("{}/download_summary.tsv", results_dir);
    let cli = Cli::parse();
    match cli.download_only {
        true => download_obo_onts(
            &cache_dir,
            &unparseable_cache_dir,
            &summary_path,
            cli.lazy,
            cli.test_length,
        ),
        false => {
            match cli.download {
                true => download_obo_onts(
                    &cache_dir,
                    &unparseable_cache_dir,
                    &summary_path,
                    cli.lazy,
                    cli.test_length,
                ),
                false => (),
            };
            let cob_purl = String::from("http://purl.obolibrary.org/obo/cob.owl");
            let cob_path = format!("{}/cob.owl", cache_dir);
            let output_path = format!("{}/obo_classes.tsv", results_dir);
            download(cob_purl, Path::new(&cob_path)).expect(&format!("Couldn't download cob.owl"));
            generate_class_tsv(&cob_path, &output_path);
        }
    };
}
