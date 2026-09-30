# COB Alignment Report for CVDO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 736 | 705 | 95.79% |
| Classes in CVDO namespace | 290 | 288 | 99.31% |

CVDO has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align CVDO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in CVDO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| CVDO:0000193 | pressure | No | 1 | BFO:0000019 |  |
