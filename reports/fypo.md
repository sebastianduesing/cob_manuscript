# COB Alignment Report for FYPO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 10224 | 5 | 0.05% |
| Classes in FYPO namespace | 8321 | 0 | 0.00% |

FYPO has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align FYPO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in FYPO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/FYPO_0000001 | phenotype | Yes | 8311 |
| http://purl.obolibrary.org/obo/FYPO_0000002 | cell phenotype | No | 9 |
