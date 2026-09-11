# COB Alignment Report for VBO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 20393 | 20169 | 98.90% |
| Classes in VBO namespace | 19902 | 19883 | 99.90% |

VBO has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align VBO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in VBO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/VBO_0300000 | breed status | Yes | 18 |
