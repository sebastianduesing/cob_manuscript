# COB Alignment Report for BSPO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 168 | 62 | 36.90% |
| Classes in BSPO namespace | 139 | 43 | 30.94% |

BSPO has 3 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align BSPO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in BSPO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/BSPO_0000051 | anatomical gradient | No | 7 |
| http://purl.obolibrary.org/obo/BSPO_0000070 | anatomical region | No | 83 |
| http://purl.obolibrary.org/obo/BSPO_0000086 | anatomical compartment | No | 5 |
