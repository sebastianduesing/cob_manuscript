# COB Alignment Report for MONDO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 58985 | 45391 | 76.95% |
| Classes in MONDO namespace | 32102 | 32044 | 99.82% |

MONDO has 2 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align MONDO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in MONDO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/MONDO_0021125 | disease characteristic | Yes | 40 |
| http://purl.obolibrary.org/obo/MONDO_0021178 | injury | Yes | 17 |
