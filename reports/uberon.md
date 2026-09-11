# COB Alignment Report for UBERON

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 25367 | 24286 | 95.74% |
| Classes in UBERON namespace | 14975 | 14971 | 99.97% |

UBERON has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align UBERON with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in UBERON. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/UBERON_0035943 | life cycle temporal boundary | Yes | 3 |
