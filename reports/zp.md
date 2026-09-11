# COB Alignment Report for ZP

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 59817 | 9229 | 15.43% |
| Classes in ZP namespace | 43522 | 0 | 0.00% |

ZP has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align ZP with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in ZP. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/ZP_0000000 | Zebrafish Phenotype | Yes | 43521 |
