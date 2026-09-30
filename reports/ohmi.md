# COB Alignment Report for OHMI

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1720 | 1662 | 96.63% |
| Classes in OHMI namespace | 896 | 877 | 97.88% |

OHMI has 12 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OHMI with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OHMI. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| OHMI:0000032 | increased organism quantity compared with healthy control | No | 0 | BFO:0000019 |  |
| OHMI:0000033 | descreased microbiome organism diversity | No | 0 | BFO:0000019 |  |
| OHMI:0000035 | decreased organism quantity compared with healthy controls | No | 0 | BFO:0000019 |  |
| OHMI:0000046 | decreased organism quantity in acute disease patient compared with chronic disease control | No | 0 | BFO:0000019 |  |
| OHMI:0000052 | decreased organism quantity compared with fibromyalgia patients | No | 0 | BFO:0000019 |  |
| OHMI:0000461 | dysbiosis | No | 0 | BFO:0000019 |  |
| OHMI:0000462 | invisible to unaided eye | No | 0 | BFO:0000019 |  |
| OHMI:0000466 | microbial species diversity | No | 0 | BFO:0000019 |  |
| OHMI:0000468 | relative species abundance | No | 1 | BFO:0000019 |  |
| OHMI:0000490 | severity of COPD | No | 4 | BFO:0000020 | characteristic |
| OHMI:0000665 | disease stable | No | 1 | BFO:0000020 | characteristic |
| OHMI:0000666 | disease exacerbation | No | 1 | BFO:0000020 | characteristic |
