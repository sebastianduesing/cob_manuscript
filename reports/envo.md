# COB Alignment Report for ENVO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 6482 | 6263 | 96.62% |
| Classes in ENVO namespace | 3872 | 3806 | 98.30% |

ENVO has 19 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align ENVO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in ENVO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| ENVO:01000785 | material extraction process | No | 1 | BFO:0000003 | process |
| ENVO:01000993 | manufacturing process | No | 1 | BFO:0000003 | process |
| ENVO:01000996 | human-directed construction process | No | 0 | BFO:0000003 | process |
| ENVO:01001303 | environmental role | No | 0 |  |  |
| ENVO:01001436 | planned environmental usage process | No | 32 | BFO:0000003 | process |
| ENVO:01001492 | satellite imaging | No | 1 | BFO:0000003 | process |
| ENVO:01001496 | inorganic macronutrient dissolved in ocean water | No | 0 |  |  |
| ENVO:01001641 | glaciation | No | 0 | BFO:0000038 |  |
| ENVO:01001642 | interglacial | No | 0 | BFO:0000038 |  |
| ENVO:01001643 | ice age | No | 0 | BFO:0000038 |  |
| ENVO:01001866 | well intervention | No | 2 | BFO:0000003 | process |
| ENVO:01001877 | transient tracer | No | 1 |  |  |
| ENVO:01003005 | day | No | 1 | BFO:0000038 |  |
| ENVO:02000007 | tissue culture | No | 4 |  |  |
| ENVO:02000146 | chemical engineering process | No | 2 |  |  |
| ENVO:02500041 | environmental monitoring | No | 1 | BFO:0000003 | process |
| ENVO:03000096 | season | No | 3 | BFO:0000038 |  |
| ENVO:06105267 | soil profile characterization | No | 0 |  |  |
| ENVO:09200037 | hours of sunshine | No | 0 | BFO:0000038 |  |
