# COB Alignment Report for PROCO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 921 | 832 | 90.34% |
| Classes in PROCO namespace | 267 | 203 | 76.03% |

PROCO has 14 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PROCO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PROCO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| PROCO:0000010 | route selection milestone | No | 0 | BFO:0000035 |  |
| PROCO:0000012 | synthesis reaction time | No | 0 | BFO:0000008 |  |
| PROCO:0000013 | critical quality attribute | No | 0 | BFO:0000019 |  |
| PROCO:0000058 | product filing milestone | No | 1 | BFO:0000035 |  |
| PROCO:0000069 | crystalline state | No | 0 | BFO:0000019 |  |
| PROCO:0000103 | product approval milestone | No | 1 | BFO:0000035 |  |
| PROCO:0000106 | induction period | No | 0 | BFO:0000008 |  |
| PROCO:0000109 | unit cell quality | No | 11 | BFO:0000019 |  |
| PROCO:0000111 | point symmetry | No | 5 | BFO:0000019 |  |
| PROCO:0000137 | habit | No | 0 | BFO:0000019 |  |
| PROCO:0000152 | point group symmetry | No | 32 | BFO:0000019 |  |
| PROCO:0000187 | bulk substance quality | No | 0 | BFO:0000019 |  |
| PROCO:0000199 | chemical structure | No | 0 | BFO:0000019 |  |
| PROCO:0000208 | molecular formula | No | 0 | BFO:0000019 |  |
