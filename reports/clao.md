# COB Alignment Report for CLAO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1518 | 0 | 0.00% |
| Classes in CLAO namespace | 1512 | 0 | 0.00% |

CLAO has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align CLAO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in CLAO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/CLAO_0001253 | molecular entity | No | 15 |  |  |
| http://purl.obolibrary.org/obo/CLAO_0001571 | entity | No | 1497 | http://purl.obolibrary.org/obo/http://purl.obolibrary.org/obo/BFO_0000004 |  |
