# COB Alignment Report for CTO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 298 | 263 | 88.26% |
| Classes in CTO namespace | 101 | 89 | 88.12% |

CTO has 4 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align CTO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in CTO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| CTO:0000201 | clinical trial phase | No | 9 | BFO:0000019 |  |
| CTO:0000214 | study start date | No | 0 | BFO:0000148 |  |
| CTO:0000215 | clinical trial primary completion date | No | 0 | BFO:0000148 |  |
| CTO:0000216 | clinical trial study completion date | No | 0 | BFO:0000148 |  |
