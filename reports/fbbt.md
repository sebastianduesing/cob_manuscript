# COB Alignment Report for FBBT

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 28252 | 20143 | 71.30% |
| Classes in FBBT namespace | 27141 | 19442 | 71.63% |

FBBT has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align FBBT with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in FBBT. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/FBbt_00005099 | neuron projection bundle | No | 599 |
| http://purl.obolibrary.org/obo/FBbt_10000000 | anatomical entity | Yes | 7100 |
