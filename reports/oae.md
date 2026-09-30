# COB Alignment Report for OAE

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 10474 | 10391 | 99.21% |
| Classes in OAE namespace | 9284 | 9223 | 99.34% |

OAE has 31 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OAE with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OAE. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| OAE:0000065 | adverse event incubation time | No | 0 | BFO:0000038 |  |
| OAE:0000071 | time at medical intervention | No | 0 | BFO:0000148 |  |
| OAE:0000072 | time instant of an adverse event outcome observed | No | 0 | BFO:0000148 |  |
| OAE:0000128 | causality of adverse event | No | 22 | BFO:0000019 |  |
| OAE:0000225 | drug resistance | No | 0 | BFO:0000019 |  |
| OAE:0000682 | pregnancy test positive | No | 0 | BFO:0000019 |  |
| OAE:0000725 | blood calcium decreased | No | 0 | BFO:0000019 |  |
| OAE:0000726 | blood chloride decreased | No | 0 | BFO:0000019 |  |
| OAE:0000738 | blood magnesium decreased | No | 0 | BFO:0000019 |  |
| OAE:0000740 | blood phosphorus decreased | No | 0 | BFO:0000019 |  |
| OAE:0000741 | blood potassium decreased | No | 1 | BFO:0000019 |  |
| OAE:0000744 | blood sodium decreased | No | 0 | BFO:0000019 |  |
| OAE:0001575 | blood phosphorus increased | No | 0 | BFO:0000019 |  |
| OAE:0001578 | blood potassium increased | No | 0 | BFO:0000019 |  |
| OAE:0001580 | blood sodium increased | No | 0 | BFO:0000019 |  |
| OAE:0001582 | blood magnesium increased | No | 0 | BFO:0000019 |  |
| OAE:0001584 | blood chloride increased | No | 0 | BFO:0000019 |  |
| OAE:0001586 | blood calcium increased | No | 0 | BFO:0000019 |  |
| OAE:0001750 | severity of adverse event | No | 6 | BFO:0000019 |  |
| OAE:0001803 | drug level increase | No | 0 | BFO:0000019 |  |
| OAE:0001804 | drug toxicity | No | 0 | BFO:0000019 |  |
| OAE:0001810 | time interval for initial process after medical intervention | No | 0 | BFO:0000038 |  |
| OAE:0001811 | time interval for intermediate causal AE process | No | 0 | BFO:0000038 |  |
| OAE:0001812 | time interval for AE process showing clinical outcome | No | 0 | BFO:0000038 |  |
| OAE:0001816 | adverse event time period | No | 1 | BFO:0000038 |  |
| OAE:0001867 | AE associated dose range | No | 0 | BFO:0000019 |  |
| OAE:0002597 | blood glucose decreased | No | 0 | BFO:0000019 |  |
| OAE:0002598 | blood glucose increased | No | 0 | BFO:0000019 |  |
| OAE:0003230 | behavior abnormal quality | No | 0 | BFO:0000019 |  |
| OAE:0004442 | adverse event start date | No | 0 | BFO:0000148 |  |
| OAE:0004449 | adverse event end date | No | 0 | BFO:0000148 |  |
