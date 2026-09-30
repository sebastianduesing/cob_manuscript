# COB Alignment Report for OPMI

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 4038 | 3857 | 95.52% |
| Classes in OPMI namespace | 711 | 604 | 84.95% |

OPMI has 32 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OPMI with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OPMI. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| OPMI:0000070 | month of year | No | 12 | BFO:0000038 |  |
| OPMI:0000083 | day of week | No | 7 | BFO:0000038 |  |
| OPMI:0000160 | body weight | No | 6 | BFO:0000019 |  |
| OPMI:0000285 | visit temporal region | No | 4 | BFO:0000148 |  |
| OPMI:0000295 | study start date | No | 1 | BFO:0000148 |  |
| OPMI:0000297 | study completion date | No | 2 | BFO:0000148 |  |
| OPMI:0000302 | date of registration | No | 0 | BFO:0000148 |  |
| OPMI:0000313 | today&apos;s date | No | 0 | BFO:0000148 |  |
| OPMI:0000322 | date of test | No | 3 | BFO:0000148 |  |
| OPMI:0000325 | sign date | No | 0 | BFO:0000148 |  |
| OPMI:0000344 | quality of life | No | 5 | BFO:0000019 |  |
| OPMI:0000353 | year | No | 0 | BFO:0000038 |  |
| OPMI:0000367 | clinical trial phase | No | 5 | BFO:0000019 |  |
| OPMI:0000478 | birth-related time | No | 0 | BFO:0000038 |  |
| OPMI:0000489 | visit duration | No | 0 | BFO:0000038 |  |
| OPMI:0000507 | procedure temporal region | No | 4 | BFO:0000148 |  |
| OPMI:0000528 | condition temporal region | No | 4 | BFO:0000148 |  |
| OPMI:0000556 | device exposure temporal region | No | 4 | BFO:0000148 |  |
| OPMI:0000564 | drug exposure temporal region | No | 6 | BFO:0000148 |  |
| OPMI:0000576 | observation temporal region | No | 2 | BFO:0000008 |  |
| OPMI:0000579 | measurement time | No | 2 | BFO:0000148 |  |
| OPMI:0000584 | payer plan temporal region | No | 3 | BFO:0000008 |  |
| OPMI:0000629 | episode start date | No | 0 | BFO:0000148 |  |
| OPMI:0000630 | episode end date | No | 0 | BFO:0000148 |  |
| OPMI:0000631 | episode start datetime | No | 0 | BFO:0000148 |  |
| OPMI:0000632 | episode end datetime | No | 0 | BFO:0000148 |  |
| OPMI:0000707 | death-related time | No | 0 | BFO:0000038 |  |
| OPMI:0004475 | age less than 18 years | No | 0 | BFO:0000019 |  |
| OPMI:0004482 | age at diagnosis of disease | No | 2 | BFO:0000019 |  |
| OPMI:0004486 | time of biopsy | No | 3 | BFO:0000148 |  |
| OPMI:0004491 | age at the start of disease | No | 2 | BFO:0000019 |  |
| OPMI:0004500 | age at the start of medication | No | 1 | BFO:0000019 |  |
