# COB Alignment Report for ENVO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 6482 | 6263 | 96.62% |
| Classes in ENVO namespace | 3872 | 3806 | 98.30% |

ENVO has 19 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align ENVO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in ENVO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/ENVO_01000785 | material extraction process | No | 1 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01000993 | manufacturing process | No | 1 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01000996 | human-directed construction process | No | 0 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01001303 | environmental role | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ENVO_01001436 | planned environmental usage process | No | 32 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01001492 | satellite imaging | No | 1 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01001496 | inorganic macronutrient dissolved in ocean water | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ENVO_01001641 | glaciation | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/ENVO_01001642 | interglacial | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/ENVO_01001643 | ice age | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/ENVO_01001866 | well intervention | No | 2 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_01001877 | transient tracer | No | 1 |  |  |
| http://purl.obolibrary.org/obo/ENVO_01003005 | day | No | 1 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/ENVO_02000007 | tissue culture | No | 4 |  |  |
| http://purl.obolibrary.org/obo/ENVO_02000146 | chemical engineering process | No | 2 |  |  |
| http://purl.obolibrary.org/obo/ENVO_02500041 | environmental monitoring | No | 1 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/ENVO_03000096 | season | No | 3 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/ENVO_06105267 | soil profile characterization | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ENVO_09200037 | hours of sunshine | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
