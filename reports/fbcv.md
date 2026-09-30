# COB Alignment Report for FBCV

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 2138 | 15 | 0.70% |
| Classes in FBCV namespace | 1194 | 0 | 0.00% |

FBCV has 3 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align FBCV with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in FBCV. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| FBcv:0000000 | FlyBase CV | Yes | 1191 |  |  |
| FBcv:0006003 | population of Drosophila | No | 0 |  |  |
| FBcv:0006007 | population of cells | No | 0 |  |  |
