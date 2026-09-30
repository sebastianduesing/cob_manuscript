# COB Alignment Report for OMRSE

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 465 | 430 | 92.47% |
| Classes in OMRSE namespace | 285 | 273 | 95.79% |

OMRSE has 5 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OMRSE with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OMRSE. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/OMRSE_00000084 | enrollment end date | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/OMRSE_00000097 | enrollment start date | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/OMRSE_00000242 | intimate partnership | No | 0 | http://purl.obolibrary.org/obo/BFO_0000145 |  |
| http://purl.obolibrary.org/obo/OMRSE_00000277 | subjective representation | No | 7 | http://purl.obolibrary.org/obo/BFO_0000020 | characteristic |
| http://purl.obolibrary.org/obo/OMRSE_00002060 | family relationship | No | 0 | http://purl.obolibrary.org/obo/BFO_0000145 |  |
