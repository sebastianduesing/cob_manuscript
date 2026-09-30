# COB Alignment Report for OHD

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1780 | 1666 | 93.60% |
| Classes in OHD namespace | 869 | 863 | 99.31% |

OHD has 6 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OHD with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OHD. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| OHD:0000211 | dental practice facility | No | 0 | BFO:0000004 |  |
| OHD:0001076 | sodium oxide | No | 0 |  |  |
| OHD:0001077 | barium oxide | No | 0 |  |  |
| OHD:0001079 | zirconium oxide | No | 0 |  |  |
| OHD:0001080 | yttrium oxide | No | 0 |  |  |
| OHD:0001081 | lithium oxide | No | 0 |  |  |
