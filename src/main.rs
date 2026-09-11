use clap::{Parser, Subcommand};
use csv::{ReaderBuilder, Writer};
use reqwest::Error;
use serde::Deserialize;
use serde_yaml::{Value, from_str};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, remove_file},
    io::{self, Write, copy},
    path::{Path, PathBuf},
};
use tabld::{
    model::{CLASS, Graph, IndexedMemoryGraph, ONTOLOGY, Subject},
    rdfxml,
};

// CLI setup
#[derive(Parser, Debug)]
#[command(name = "cob_align", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    // Downloads active OBO ontologies
    Download {
        // Don't download files that are already in cache/ or unparseable/
        #[arg(short = 'l', long = "lazy")]
        lazy: bool,

        // Attempt to download only this number of ontologies
        #[arg(short = 't', long = "test-length")]
        test_length: Option<u16>,
    },
    // Checks downloaded ontologies for COB alignment
    Analyze {},
    // Creates per-ontology alignment reports
    Report {},
}

struct Ontology {
    id: String,
    class_count: u64,
    ns_class_count: u64,
    aligned_class_count: u64,
    aligned_ns_class_count: u64,
    unaligned_roots: BTreeMap<String, Root>,
}

struct Root {
    _id: String,
    label: String,
    desc_count: u64,
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
    lazy: &bool,
    test_length: &Option<u16>,
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
                if &downloads >= int {
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
        if *lazy {
            if path.exists() || rdfxml_error_path.exists() {
                let mut dl_status = "Cached";
                if rdfxml_error_path.exists() {
                    dl_status = "Cached (unparseable)"
                }
                eprintln!("Already downloaded {filename}");
                downloads += 1;
                wtr.write_record([
                    ont.get("id").unwrap(),
                    ont.get("purl").unwrap(),
                    ont.get("activity_status").unwrap(),
                    dl_status,
                ])
                .unwrap();
                continue;
            }
        }
        let purl = String::from(ont.get("purl").unwrap());
        let mut dl_status = "Download not attempted";
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
                            dl_status = "Download failed";
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
fn check_class_alignment(
    ont_path: PathBuf,
    class_wtr: &mut Writer<File>,
    analysis_wtr: &mut Writer<File>,
    roots_wtr: &mut Writer<File>,
    cob_subjects: &Vec<&Subject>,
) -> Result<(), String> {
    let ont = ont_path.file_prefix().unwrap().display();
    let ont_string = format!("{ont}");
    let iri_flag = format!("/{}_", ont_string.to_lowercase()).to_string();
    eprintln!("Analyzing classes in {ont_string}");

    let rdfxml_input = match std::fs::read_to_string(ont_path) {
        Ok(string) => string,
        Err(err) => {
            return Err(format!("Could not read {ont_string}: {err:?}"));
        }
    };
    let mut ontology = Ontology {
        id: ont_string.clone(),
        class_count: 0,
        ns_class_count: 0,
        aligned_class_count: 0,
        aligned_ns_class_count: 0,
        unaligned_roots: BTreeMap::new(),
    };
    let mut preferred_roots: BTreeSet<String> = BTreeSet::new();
    let graph = rdfxml::read(&rdfxml_input).expect("Read from string");
    // todo: should move files to unparseable/ automatically when tabld cannot read them
    // but tabld currently panics when it can't read the rdfxml in a file
    // so that change is pending different error handling in tabld
    let graph: IndexedMemoryGraph = graph.into();

    for subject in graph.subjects() {
        if let Some(ONTOLOGY) = subject.owl_type() {
            if subject
                .predicates()
                .contains_key("http://purl.obolibrary.org/obo/IAO_0000700")
            {
                match subject
                    .predicates()
                    .get("http://purl.obolibrary.org/obo/IAO_0000700")
                {
                    Some(val) => {
                        for obj in val {
                            preferred_roots.insert(obj.object());
                        }
                    }
                    None => (),
                };
            }
        }
        let mut in_base = "False";
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
        ontology.class_count = ontology.class_count + 1;
        if subject.name().to_lowercase().contains(&iri_flag) {
            in_base = "True";
            ontology.ns_class_count = ontology.ns_class_count + 1;
        }
        let mut term_ancestors = graph.ancestors(&subject.name());
        let name = subject.name();
        term_ancestors.insert(&name);
        let mut found: bool = false;
        for cob_subject in cob_subjects.iter() {
            if term_ancestors.contains(&cob_subject.name()) {
                ontology.aligned_class_count = ontology.aligned_class_count + 1;
                if in_base == "True" {
                    ontology.aligned_ns_class_count = ontology.aligned_ns_class_count + 1;
                }
                class_wtr
                    .write_record([
                        &subject.name(),
                        &subject.label().replace("\n", "").replace("\t", ""),
                        &cob_subject.name(),
                        &cob_subject.label(),
                        &ont_string,
                        in_base,
                        "",
                        "",
                    ])
                    .unwrap();
                found = true;
                break;
            }
        }
        if !found {
            let mut top_ns_ancestor = "".to_string();
            let mut root_name = "".to_string();
            let mut top_anc_label = "".to_string();
            if in_base == "True" {
                let mut anc_vec: Vec<&String> = term_ancestors.into_iter().collect();
                anc_vec.sort_by_key(|s| graph.ancestors(s).len());
                anc_vec.reverse();
                if anc_vec.len() > 0 {
                    for ancestor in anc_vec.iter() {
                        if ancestor.to_lowercase().contains(&iri_flag) {
                            top_ns_ancestor = ancestor.to_string();
                        } else {
                            break;
                        }
                    }
                }
                root_name = if top_ns_ancestor != "".to_string() {
                    top_ns_ancestor.clone()
                } else {
                    subject.name().clone()
                };
                if let Some(root) = ontology.unaligned_roots.get_mut(&root_name) {
                    top_anc_label = root.label.clone();
                    root.desc_count += 1;
                } else {
                    if let Some(subj) = graph.get(&root_name) {
                        top_anc_label = subj.label().clone();
                    };
                    let desc_count = if subject.name() == root_name.to_string() {
                        0
                    } else {
                        1
                    };
                    let root = Root {
                        _id: root_name.to_string(),
                        label: top_anc_label.to_string(),
                        desc_count: desc_count,
                    };
                    ontology.unaligned_roots.insert(root_name.to_string(), root);
                }
            }
            class_wtr
                .write_record([
                    &subject.name(),
                    &subject.label().replace("\n", "").replace("\t", ""),
                    "",
                    "",
                    &ont_string,
                    in_base,
                    &root_name,
                    &top_anc_label,
                ])
                .unwrap();
        }
    }
    let mut ns_ratio = "".to_string();
    let mut aligned_ratio = "".to_string();
    let mut aligned_ns_ratio = "".to_string();
    if ontology.class_count > 0 {
        ns_ratio = (ontology.ns_class_count as f32 / ontology.class_count as f32).to_string();
        aligned_ratio =
            (ontology.aligned_class_count as f32 / ontology.class_count as f32).to_string();
    }
    if ontology.ns_class_count > 0 {
        aligned_ns_ratio =
            (ontology.aligned_ns_class_count as f32 / ontology.ns_class_count as f32).to_string()
    }
    for root in ontology.unaligned_roots.keys() {
        let mut is_preferred = "No";
        if preferred_roots.contains(root) {
            is_preferred = "Yes";
        }
        roots_wtr
            .write_record([
                ont_string.clone(),
                root.to_string(),
                ontology.unaligned_roots.get(root).unwrap().label.clone(),
                is_preferred.to_string(),
                ontology
                    .unaligned_roots
                    .get(root)
                    .unwrap()
                    .desc_count
                    .to_string(),
            ])
            .unwrap();
    }
    analysis_wtr
        .write_record([
            ontology.id,
            ontology.class_count.to_string(),
            ontology.ns_class_count.to_string(),
            ns_ratio,
            ontology.aligned_class_count.to_string(),
            aligned_ratio,
            ontology.aligned_ns_class_count.to_string(),
            aligned_ns_ratio,
            ontology.unaligned_roots.keys().len().to_string(),
        ])
        .unwrap();
    Ok(())
}

// Generate a table of classes and relevant alignment info
fn generate_class_tsv(
    cob_path: &str,
    class_tsv_path: &str,
    analysis_tsv_path: &str,
    roots_tsv_path: &str,
) {
    let rdfxml_input = std::fs::read_to_string(cob_path).expect("Read from file");
    let graph = rdfxml::read(&rdfxml_input).expect("Read from string");
    let cob_graph: IndexedMemoryGraph = graph.into();
    let mut cob_subjects: Vec<&Subject> = cob_graph.subjects().into_iter().collect();
    cob_subjects.sort_by_key(|s| cob_graph.ancestors(&s.name()).len());
    cob_subjects.reverse();

    let mut class_wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(class_tsv_path)
        .unwrap();
    class_wtr
        .write_record([
            "Class IRI",
            "Class Label",
            "Lowest COB Ancestor IRI",
            "Lowest COB Ancestor Label",
            "Found In",
            "In Namespace?",
            "Highest In-Namespace Ancestor IRI",
            "Highest In-Namespace Ancestor Label",
        ])
        .unwrap();

    let mut analysis_wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(analysis_tsv_path)
        .unwrap();
    analysis_wtr
        .write_record([
            "Ontology",
            "Total Classes",
            "Classes in Namespace",
            "Ratio of In- to Out-of-Namespace Classes",
            "Total Aligned Classes",
            "Ratio of Aligned Classes to All Classes",
            "Aligned in-Namespace Classes",
            "Ratio of Aligned in-Namespace Classes to All in-Namespace Classes",
            "Unaligned Roots",
        ])
        .unwrap();

    let mut roots_wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::Never)
        .from_path(roots_tsv_path)
        .unwrap();
    roots_wtr
        .write_record([
            "Ontology",
            "Root IRI",
            "Root Label",
            "Is Preferred Root?",
            "Descendent Class Count",
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
        check_class_alignment(
            e,
            &mut class_wtr,
            &mut analysis_wtr,
            &mut roots_wtr,
            &cob_subjects,
        )
        .expect("couldn't print terms");
    }
}

#[derive(Debug, Deserialize)]
struct AnalysisRecord {
    #[serde(rename = "Ontology")]
    ont_name: String,
    #[serde(rename = "Total Classes")]
    classes: u64,
    #[serde(rename = "Classes in Namespace")]
    ns_classes: u64,
    #[serde(rename = "Ratio of In- to Out-of-Namespace Classes")]
    _ns_ratio: f64,
    #[serde(rename = "Total Aligned Classes")]
    aligned_classes: u64,
    #[serde(rename = "Ratio of Aligned Classes to All Classes")]
    aligned_all_ratio: f64,
    #[serde(rename = "Aligned in-Namespace Classes")]
    aligned_ns_classes: u64,
    #[serde(
        rename = "Ratio of Aligned in-Namespace Classes to All in-Namespace Classes",
        deserialize_with = "csv::invalid_option"
    )]
    aligned_ns_all_ns_ratio: Option<f64>,
    #[serde(rename = "Unaligned Roots")]
    _roots: u64,
}

#[derive(Debug, Deserialize)]
struct RootRecord {
    #[serde(rename = "Ontology")]
    ontology: String,
    #[serde(rename = "Root IRI")]
    iri: String,
    #[serde(rename = "Root Label")]
    label: String,
    #[serde(rename = "Is Preferred Root?")]
    preferred: String,
    #[serde(rename = "Descendent Class Count")]
    desc_count: u64,
}

fn report(
    // class_tsv_path: &str,
    analysis_tsv_path: &str,
    roots_tsv_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let reports_dir = "reports/";
    if !Path::new(reports_dir).exists() {
        fs::create_dir("reports").expect("Failed to create reports dir");
        eprintln!("Created directory: reports/")
    }

    let mut roots_rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(roots_tsv_path)?;
    let mut ontology_roots: BTreeMap<String, Vec<RootRecord>> = BTreeMap::new();
    for result in roots_rdr.deserialize() {
        let record: RootRecord = result?;
        let root_vec = ontology_roots
            .entry(record.ontology.clone())
            .or_insert(Vec::new());
        root_vec.insert(root_vec.len(), record);
    }

    let mut analysis_rdr = ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(analysis_tsv_path)?;
    for result in analysis_rdr.deserialize() {
        let record: AnalysisRecord = result?;
        let roots = match ontology_roots.get(&record.ont_name) {
            Some(roots) => roots,
            None => &Vec::new(),
        };
        let report_path = format!("{}/{}.md", reports_dir, &record.ont_name);
        let mut f = File::create(report_path)?;
        let name = &record.ont_name.to_uppercase();

        write!(f, "# COB Alignment Report for {}\n\n", name)?;
        write!(
            f,
            "In the table below, \"aligned classes\" are classes that have at least one ancestor that is a term in COB.\n\n"
        )?;
        write!(
            f,
            "| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |\n"
        )?;
        write!(f, "| ----- | ----- | ----- | ----- |\n")?;
        write!(
            f,
            "| All classes (including imports) | {:?} | {:?} | {:.2}% |\n",
            record.classes,
            record.aligned_classes,
            record.aligned_all_ratio * 100.0
        )?;
        let ns_ratio = match record.aligned_ns_all_ns_ratio {
            Some(n) => format!("{:.2}%", n * 100.0),
            None => "N/A (no in-namespace classes)".to_string(),
        };
        write!(
            f,
            "| Classes in {} namespace | {:?} | {:?} | {} |\n\n",
            name, record.ns_classes, record.aligned_ns_classes, ns_ratio
        )?;
        let root_num = roots.iter().len();
        let instructions = if root_num == 0 {
            format!("{name} is fully aligned with COB.")
        } else {
            format!(
                "To align {name} with COB, these terms should be moved under COB terms or added to COB."
            )
        };
        write!(
            f,
            "{name} has {} unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. {instructions}\n\n",
            roots.iter().len(),
        )?;
        let pref_root_desc = "The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.";
        if root_num > 0 {
            if root_num <= 100 {
                write!(
                    f,
                    "The table below contains all unaligned roots in {name}. {pref_root_desc}\n\n"
                )?;
            } else {
                write!(
                    f,
                    "The table below contains the first 100 unaligned roots in {name}. {pref_root_desc}\n\n"
                )?;
            };
            write!(
                f,
                "| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |\n"
            )?;
            write!(f, "| ----- | ----- | ----- | ----- |\n")?;
            let mut count = 0;
            for root in roots.iter() {
                count += 1;
                if count > 100 {
                    break;
                }
                write!(
                    f,
                    "| {} | {} | {} | {} |\n",
                    root.iri, root.label, root.preferred, root.desc_count
                )?
            }
        }
    }

    Ok(())
}

fn main() {
    let cache_dir = "cache";
    let unparseable_cache_dir = "unparseable";
    let results_dir = "results";
    let class_tsv_path = format!("{}/obo_classes.tsv", results_dir);
    let analysis_tsv_path = format!("{}/alignment_analysis.tsv", results_dir);
    let roots_tsv_path = format!("{}/unaligned_roots.tsv", results_dir);
    let cli = Cli::parse();
    match &cli.command {
        Commands::Download { lazy, test_length } => {
            if !Path::new(cache_dir).exists() {
                fs::create_dir("cache").expect("Failed to create cache");
                eprintln!("Created directory: cache/")
            }
            if !Path::new(unparseable_cache_dir).exists() {
                fs::create_dir("unparseable").expect("Failed to create unparseable file dir");
                eprintln!("Created directory: unparseable/")
            }
            let summary_path = format!("{}/download_summary.tsv", results_dir);
            download_obo_onts(
                &cache_dir,
                &unparseable_cache_dir,
                &summary_path,
                lazy,
                test_length,
            );
        }
        Commands::Analyze {} => {
            if !Path::new(cache_dir).exists() {
                panic!("No cache found. Run 'cargo run -- download' to cache files")
            }
            if !Path::new(unparseable_cache_dir).exists() {
                fs::create_dir("unparseable").expect("Failed to create unparseable file dir");
            }
            if !Path::new(results_dir).exists() {
                fs::create_dir("results").expect("Failed to create results dir");
                eprintln!("Created directory: results/")
            }
            let cob_purl = String::from("http://purl.obolibrary.org/obo/cob.owl");
            let cob_path = format!("{}/cob.owl", cache_dir);
            download(cob_purl, Path::new(&cob_path)).expect(&format!("Couldn't download cob.owl"));
            generate_class_tsv(
                &cob_path,
                &class_tsv_path,
                &analysis_tsv_path,
                &roots_tsv_path,
            );
        }
        Commands::Report {} => {
            if !Path::new(results_dir).exists() {
                panic!(
                    "No analysis files found. \
                    Run 'cargo run -- download' to cache files and \
                    run 'cargo run -- analyze' to create analysis tables"
                )
            }
            if !Path::new(&class_tsv_path).exists()
                || !Path::new(&analysis_tsv_path).exists()
                || !Path::new(&roots_tsv_path).exists()
            {
                panic!(
                    "Missing some analysis file(s). \
                    Run 'cargo run -- analyze' to create analysis tables"
                )
            }
            report(&analysis_tsv_path, &roots_tsv_path).expect("Failed to generate reports");
        }
    }
}
