# COB Alignment Report for FBBI

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 581 | 16 | 2.75% |
| Classes in FBBI namespace | 556 | 0 | 0.00% |

FBBI has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align FBBI with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in FBBI. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/FBbi_root_00000000 | method involved in biological imaging | Yes | 556 |
