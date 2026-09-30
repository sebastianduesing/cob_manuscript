# COB Alignment Report for OBA

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 87766 | 49825 | 56.77% |
| Classes in OBA namespace | 25344 | 114 | 0.45% |

OBA has 1 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OBA with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OBA. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/OBA_0000001 | biological attribute | Yes | 25229 | http://purl.obolibrary.org/obo/BFO_0000020 | characteristic |
