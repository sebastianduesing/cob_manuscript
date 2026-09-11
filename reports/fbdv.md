# COB Alignment Report for FBDV

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 239 | 47 | 19.67% |
| Classes in FBDV namespace | 210 | 21 | 10.00% |

FBDV has 4 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align FBDV with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in FBDV. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/FBdv_00000000 | Drosophila life | Yes | 0 |
| http://purl.obolibrary.org/obo/FBdv_00005259 | developmental stage | Yes | 111 |
| http://purl.obolibrary.org/obo/FBdv_00007013 | age | Yes | 72 |
| http://purl.obolibrary.org/obo/FBdv_00007024 | biological process | Yes | 4 |
