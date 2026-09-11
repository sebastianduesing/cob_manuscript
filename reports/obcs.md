# COB Alignment Report for OBCS

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1210 | 1177 | 97.27% |
| Classes in OBCS namespace | 370 | 368 | 99.46% |

OBCS has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OBCS with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OBCS. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/OBCS_0000231 | mortality rate | No | 0 |
| http://purl.obolibrary.org/obo/OBCS_0000232 | hospital unit  shift rate | No | 0 |
