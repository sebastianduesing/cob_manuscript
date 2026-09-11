# COB Alignment Report for OGMS

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 185 | 156 | 84.32% |
| Classes in OGMS namespace | 117 | 101 | 86.32% |

OGMS has 8 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align OGMS with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in OGMS. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/OGMS_0000022 | manifestation of a disease | No | 2 |
| http://purl.obolibrary.org/obo/OGMS_0000023 | phenotype | No | 2 |
| http://purl.obolibrary.org/obo/OGMS_0000039 | configuration | No | 1 |
| http://purl.obolibrary.org/obo/OGMS_0000067 | _undefined primitive term | No | 3 |
| http://purl.obolibrary.org/obo/OGMS_0000074 | normal value | No | 0 |
| http://purl.obolibrary.org/obo/OGMS_0000086 | syndrome | No | 0 |
| http://purl.obolibrary.org/obo/OGMS_0000122 | recovered from disease | No | 0 |
| http://purl.obolibrary.org/obo/OGMS_0000142 | qualitative sign | No | 0 |
