# COB Alignment Report for ADO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1963 | 1898 | 96.69% |
| Classes in ADO namespace | 53 | 49 | 92.45% |

ADO has 4 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align ADO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in ADO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/ADO_0000010 | TM_BIN_VariantisGeneticRiskFactorFor | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ADO_0000011 | TM_BIN_isSignAndSymptomFor | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ADO_0000012 | TM_BIN_isTreatmentFor | No | 0 |  |  |
| http://purl.obolibrary.org/obo/ADO_0000024 | early sign | No | 0 | http://purl.obolibrary.org/obo/BFO_0000001 |  |
