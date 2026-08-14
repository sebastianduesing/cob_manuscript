# Scripts and Data for the 2026 COB Manuscript

The scripts in this repository gather and analyze the two corpora of data used in the upcoming 2026 manuscript on the Core Ontology for Biology and Biomedicine (COB). The first corpus of data is an analysis of classes in OBO Foundry ontologies conducted to identify ontologies' alignment with COB. The second corpus of data is the result of a survey of biologists, data scientists, and ontologists about their intuitions regarding classification of a set of randomly sampled biological terms.

## COB Alignment Data

The alignment analysis script downloads all active OBO ontologies and reads through the files, examining each class for whether it has an ancestor that is a term in COB, and if not, what its highest in-namespace ancestor is.

Running the alignment-analysis script requires installing [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). With Rust installed, run the following command from within the repository:

```
cargo run --release -- -d
```

`-d` or `--download` toggles on the part of the script that downloads active OBO Foundry ontologies and saves them in a directory called `cache/`. Without including the `-d` flag, the script proceeds to attempting to analyze files in the `cache/` directory, so if you have the files downloaded and just want to rerun the analysis, run `cargo run --release`.


`-l` or `--lazy` makes the script skip downloading ontologies that already exist in the `cache/` directory (or in a directory called `unparseable/`, which I will discuss shortly). If you get part of the way through downloading ontologies and must restart the script, use `-l` to avoid redownloading what you've already got.

`-o` or `--download-only` makes the script attempt downloads as needed, but skip the analysis.

`-t <n>` or `--test-length <n>` makes the script stop attempting to download files after doing so for _n_ files.

On my machine, downloading all active OBO ontologies takes about 5 minutes, and running the analysis portion of the script takes under 3 minutes.

### RDFXML Parsing Errors

While running the analysis portion of the script, the script may abort with an RDFXML parsing error. When this occurs, move the file mentioned in the last `Analyzing classes in <ont>` printout from the `cache/` directory into the `unparseable/` directory, and then rerun the script.

As of August 2026, when I run this script, the files I find that I need to move into `unparseable/` are `cto.owl`, `gaz.owl`, `ogg.owl`, and `sbo.owl`.

### Results Files

The results of the script end up in [`results/`](results/).

[`results/alignment_analysis.tsv`](results/alignment_analysis.tsv) is a table of, for each ontology analyzed, the number of classes in and out of that ontology's namespace and in and out of alignment with COB (i.e., having or not having a COB ancestor).

[`results/download_summary.tsv`](results/download_summary.tsv) is a table of the ontologies the script found in the OBO registry, their activity status, and whether it downloaded, skipped, or did not attempt to download that ontology.

[`results/obo_classes.tsv`](results/obo_classes.tsv) is a table of every class in every OBO Foundry ontology the script downloaded and analyzed, including for each class its IRI & label, the IRI & label of its lowest COB ancestor if one exists, the ontology in which that class was found, whether it is in that ontology's namespace, and if so and if it has no COB ancestor, what its highest in-namespace ancestor is.

[`results/unaligned_roots.tsv`](results/unaligned_roots.tsv) is a table of unaligned roots (i.e., highest in-namespace classes without COB ancestors), the ontology they are from, whether they are marked as a preferred root (via an `IAO:0000700` annotation), and how many descendent terms are under that root.
