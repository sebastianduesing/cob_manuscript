# COB Alignment Report for UBERON

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 25483 | 24352 | 95.56% |
| Classes in UBERON namespace | 15047 | 15043 | 99.97% |

UBERON has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align UBERON with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in UBERON. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| UBERON:0035943 | life cycle temporal boundary | Yes | 3 | BFO:0000001 |  |
