# COB Alignment Report for TAXRANK

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 77 | 0 | 0.00% |
| Classes in TAXRANK namespace | 77 | 0 | 0.00% |

TAXRANK has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align TAXRANK with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in TAXRANK. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| TAXRANK:0000000 | taxonomic_rank | Yes | 73 |  |  |
| TAXRANK:9000000 | pseudorank | Yes | 2 |  |  |
