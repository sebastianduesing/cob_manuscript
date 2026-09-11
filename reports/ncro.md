# COB Alignment Report for NCRO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 3078 | 2849 | 92.56% |
| Classes in NCRO namespace | 2548 | 2530 | 99.29% |

NCRO has 8 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align NCRO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in NCRO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses |
| ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/NCRO_0000001 | miRNA_target_gene_primary_transcript | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000002 | miRNA_target_gene_mRNA | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000012 | promoter_of_miRNA | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000025 | miRNA_target_gene | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000026 | environmental_variable | No | 8 |
| http://purl.obolibrary.org/obo/NCRO_0000027 | gene_sequence | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000801 | TE_juxtaposition | No | 0 |
| http://purl.obolibrary.org/obo/NCRO_0000803 | TE_insertion | No | 2 |
