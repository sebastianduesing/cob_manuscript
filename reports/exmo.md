# COB Alignment Report for EXMO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 449 | 364 | 81.07% |
| Classes in EXMO namespace | 220 | 182 | 82.73% |

EXMO has 9 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align EXMO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in EXMO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| EXMO:0000015 | exercise prescription | No | 0 | BFO:0000031 | information content entity |
| EXMO:0000016 | characteristic of exercise | No | 6 | BFO:0000019 |  |
| EXMO:0000080 | resting heart rate | No | 0 | BFO:0000019 |  |
| EXMO:0000305 | exercise addiction | No | 0 | BFO:0000019 |  |
| EXMO:0000326 | total energy expenditure | No | 1 | BFO:0000019 |  |
| EXMO:0000329 | thermic effect of food | No | 0 | BFO:0000019 |  |
| EXMO:0000379 | exercise plan | No | 0 | BFO:0000031 | information content entity |
| EXMO:0000385 | fitness | No | 21 | BFO:0000019 |  |
| EXMO:0000399 | screen time | No | 2 | BFO:0000019 |  |
