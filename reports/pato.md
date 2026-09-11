# COB Alignment Report for PATO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 7690 | 5658 | 73.58% |
| Classes in PATO namespace | 1887 | 12 | 0.64% |

PATO has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PATO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PATO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/PATO_0000001 | quality | Yes | 1874 |
