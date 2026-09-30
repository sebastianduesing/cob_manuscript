# COB Alignment Report for GENO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 540 | 277 | 51.30% |
| Classes in GENO namespace | 222 | 70 | 31.53% |

GENO has 7 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align GENO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in GENO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/GENO_0000575 | zebrafish phenotype | No | 0 | http://purl.obolibrary.org/obo/BFO_0000020 | characteristic |
| http://purl.obolibrary.org/obo/GENO_0000701 | sequence feature or set | No | 60 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/GENO_0000713 | qualified sequence feature or collection | No | 15 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/GENO_0000788 | sequence feature attribute | No | 57 | http://purl.obolibrary.org/obo/BFO_0000020 | characteristic |
| http://purl.obolibrary.org/obo/GENO_0000815 | sequence feature location | No | 1 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/GENO_0000897 | genomic entity | No | 0 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/GENO_0000921 | biological sequence or set | No | 16 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
