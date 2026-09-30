# COB Alignment Report for HANCESTRO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1263 | 1249 | 98.89% |
| Classes in HANCESTRO namespace | 589 | 584 | 99.15% |

HANCESTRO has 3 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align HANCESTRO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in HANCESTRO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| HANCESTRO:0304 | ancestry status | Yes | 2 | BFO:0000019 |  |
| HANCESTRO:0599 | ethnicity descriptor | No | 0 | BFO:0000019 |  |
| HANCESTRO:0600 | geographic descriptor | No | 0 | BFO:0000019 |  |
