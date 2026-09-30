# COB Alignment Report for IDO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 344 | 304 | 88.37% |
| Classes in IDO namespace | 253 | 227 | 89.72% |

IDO has 20 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align IDO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in IDO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| IDO:0000463 | infectious agent transmissibility | No | 0 | BFO:0000019 |  |
| IDO:0000464 | infectivity | No | 0 | BFO:0000019 |  |
| IDO:0000466 | virulence | No | 0 | BFO:0000019 |  |
| IDO:0000467 | susceptibility | No | 3 | BFO:0000019 |  |
| IDO:0000479 | infectious disease incidence | No | 0 | BFO:0000019 |  |
| IDO:0000480 | infection incidence | No | 0 | BFO:0000019 |  |
| IDO:0000481 | infectious disease incidence proportion | No | 0 | BFO:0000019 |  |
| IDO:0000482 | infection incidence proportion | No | 0 | BFO:0000019 |  |
| IDO:0000483 | infectious disease incidence rate | No | 1 | BFO:0000019 |  |
| IDO:0000484 | infection incidence rate | No | 0 | BFO:0000019 |  |
| IDO:0000485 | infectious disease prevalence | No | 1 | BFO:0000019 |  |
| IDO:0000486 | infection prevalence | No | 0 | BFO:0000019 |  |
| IDO:0000487 | infectious disease lifetime prevalence | No | 0 | BFO:0000019 |  |
| IDO:0000488 | infectious agent seroprevalence | No | 0 | BFO:0000019 |  |
| IDO:0000489 | infectious disease mortality rate | No | 0 | BFO:0000019 |  |
| IDO:0000490 | infectious disease endemicity | No | 1 | BFO:0000019 |  |
| IDO:0000494 | infectious disease sporadicity | No | 0 | BFO:0000019 |  |
| IDO:0000519 | incubation period | No | 0 | BFO:0000038 |  |
| IDO:0000520 | communicability period | No | 0 | BFO:0000038 |  |
| IDO:0000593 | transmission period | No | 0 | BFO:0000038 |  |
