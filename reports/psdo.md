# COB Alignment Report for PSDO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 96 | 68 | 70.83% |
| Classes in PSDO namespace | 93 | 67 | 72.04% |

PSDO has 10 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PSDO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PSDO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/PSDO_0000020 | scale type | No | 4 |
| http://purl.obolibrary.org/obo/PSDO_0000028 | performance trend quality | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000029 | performance gap quality | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000031 | performance report | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000061 | attribute | No | 13 |
| http://purl.obolibrary.org/obo/PSDO_0000062 | mark | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000079 | mark collection | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000098 | performance summary document | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000110 | performance summary textual entity | No | 0 |
| http://purl.obolibrary.org/obo/PSDO_0000114 | performance summary section | No | 0 |
