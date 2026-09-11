# COB Alignment Report for MRO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 51403 | 50937 | 99.09% |
| Classes in MRO namespace | 51300 | 50840 | 99.10% |

MRO has 5 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align MRO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in MRO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/MRO_0000004 | MHC gene | No | 393 |
| http://purl.obolibrary.org/obo/MRO_0000005 | MHC haplotype | No | 60 |
| http://purl.obolibrary.org/obo/MRO_0000008 | T cell epitope dependent biological activity assay | No | 0 |
| http://purl.obolibrary.org/obo/MRO_0000009 | MHC ligand assay | No | 0 |
| http://purl.obolibrary.org/obo/MRO_0037049 | Beta-2-microglobulin gene | No | 3 |
