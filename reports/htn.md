# COB Alignment Report for HTN

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 602 | 498 | 82.72% |
| Classes in HTN namespace | 21 | 14 | 66.67% |

HTN has 5 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align HTN with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in HTN. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| ----- | ----- | ----- | ----- |
| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| http://purl.obolibrary.org/obo/HTN_00000004 | documented hypertensive phenotype | No | 0 |
| http://purl.obolibrary.org/obo/HTN_00000014 | elevated blood pressure phenotype | No | 3 |
| http://purl.obolibrary.org/obo/HTN_00000035 | diagnosis of hypertension | No | 0 |
| http://purl.obolibrary.org/obo/HTN_00000040 | stage 1 elevated adult systolic blood pressure meaurement datum per ACC 2017 guidelines | No | 0 |
| http://purl.obolibrary.org/obo/HTN_00000051 | adult over 18 years of age measurement datum | No | 0 |
