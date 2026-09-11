# COB Alignment Report for ECO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 3320 | 2559 | 77.08% |
| Classes in ECO namespace | 2217 | 1483 | 66.89% |

ECO has 3 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align ECO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in ECO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/ECO_0000000 | evidence | No | 481 |
| http://purl.obolibrary.org/obo/ECO_0000006 | experimental evidence | No | 249 |
| http://purl.obolibrary.org/obo/ECO_0000217 | assertion method | No | 3 |
