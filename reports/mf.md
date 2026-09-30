# COB Alignment Report for MF

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 400 | 381 | 95.25% |
| Classes in MF namespace | 95 | 89 | 93.68% |

MF has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align MF with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in MF. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| MF:0000030 | representation | No | 2 | BFO:0000020 | characteristic |
| MF:0000074 | bodily quality | No | 3 | BFO:0000019 |  |
