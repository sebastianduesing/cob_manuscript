# COB Alignment Report for HSO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 447 | 418 | 93.51% |
| Classes in HSO namespace | 104 | 96 | 92.31% |

HSO has 3 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align HSO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in HSO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/HSO_0000059 | language | No | 0 |
| http://purl.obolibrary.org/obo/HSO_0000388 | status of hazard presence in a geographical region | No | 5 |
| http://purl.obolibrary.org/obo/HSO_0000414 | genetic epidemiology surveillance datum | No | 0 |
