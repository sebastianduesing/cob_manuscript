# COB Alignment Report for RBO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 9242 | 9078 | 98.23% |
| Classes in RBO namespace | 405 | 403 | 99.51% |

RBO has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align RBO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in RBO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/RBO_00010064 | energy deposition event | No | 0 |
| http://purl.obolibrary.org/obo/RBO_00015012 | ionization cluster | No | 0 |
