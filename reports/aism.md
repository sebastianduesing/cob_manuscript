# COB Alignment Report for AISM

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 8778 | 7383 | 84.11% |
| Classes in AISM namespace | 742 | 197 | 26.55% |

AISM has 12 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align AISM with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in AISM. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/AISM_0000005 | cuticular depression | No | 24 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000013 | evaginated | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/AISM_0000174 | insect region of cuticle | Yes | 499 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000353 | reticulate | No | 3 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/AISM_0000374 | tibial margin | No | 6 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000376 | interpunctural distance | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/AISM_0000381 | head margin at genoclypeal sulcus | No | 0 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000385 | clypeal margin | No | 3 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000386 | genal margin | No | 0 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000405 | antero-distal margin | No | 0 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0000406 | postero-distal margin | No | 0 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/AISM_0004317 | deflexed | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
