# COB Alignment Report for PDRO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 314 | 284 | 90.45% |
| Classes in PDRO namespace | 169 | 165 | 97.63% |

PDRO has 4 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PDRO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PDRO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| PDRO:0000322 | drug prescription validity period | No | 0 | BFO:0000038 |  |
| PDRO:9876001 | administration dose form | No | 0 | BFO:0000019 |  |
| PDRO:9876002 | drug product dose form | No | 0 | BFO:0000019 |  |
| PDRO:9876003 | active ingredient aggregate biological activity | No | 0 | BFO:0000019 |  |
