# COB Alignment Report for TXPO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 9410 | 9160 | 97.34% |
| Classes in TXPO namespace | 5589 | 5482 | 98.09% |

TXPO has 10 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align TXPO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in TXPO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/TXPO_0000270 | state | No | 15 | http://purl.obolibrary.org/obo/BFO_0000003 | process |
| http://purl.obolibrary.org/obo/TXPO_0000275 | quality value | No | 32 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/TXPO_0000300 | attribute | No | 50 | http://purl.obolibrary.org/obo/BFO_0000020 | characteristic |
| http://purl.obolibrary.org/obo/TXPO_0000315 | colorness (quality) | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0000316 | high brightness | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0000657 | predicted (quality) | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0002450 | decreased amount | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0002873 | chronic | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0003518 | acidophilic | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/TXPO_0003563 | increased affinity | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
