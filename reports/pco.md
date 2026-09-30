# COB Alignment Report for PCO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 203 | 142 | 69.95% |
| Classes in PCO namespace | 65 | 40 | 61.54% |

PCO has 8 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PCO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PCO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| PCO:0000003 | quality of a population | No | 4 | BFO:0000020 | characteristic |
| PCO:0000004 | quality of an ecological community | No | 11 | BFO:0000020 | characteristic |
| PCO:0000006 | population birth rate | No | 0 | BFO:0000020 | characteristic |
| PCO:0000007 | population death rate | No | 0 | BFO:0000020 | characteristic |
| PCO:0000008 | population growth rate | No | 0 | BFO:0000020 | characteristic |
| PCO:0000048 | invisible to unaided eye | No | 0 | BFO:0000020 | characteristic |
| PCO:0000050 | collection of microbial organisms | No | 1 |  |  |
| PCO:0000077 | plant density | No | 1 | BFO:0000020 | characteristic |
