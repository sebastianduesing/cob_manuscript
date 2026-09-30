# COB Alignment Report for APOLLO_SV

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 1647 | 1550 | 94.11% |
| Classes in APOLLO_SV namespace | 616 | 580 | 94.16% |

APOLLO_SV has 12 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align APOLLO_SV with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in APOLLO_SV. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| IRI of Root | Label of Root | Preferred Root? | Number of Subclasses | Lowest BFO Ancestor | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000000 | purely intentional entity | No | 17 | http://purl.obolibrary.org/obo/BFO_0000031 | information content entity |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000166 | contaminated thing | No | 0 | http://purl.obolibrary.org/obo/BFO_0000004 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000241 | age range category | No | 0 |  |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000400 | epidemic interval | No | 0 | http://purl.obolibrary.org/obo/BFO_0000038 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000430 | ring diameter in meters | No | 0 |  |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000574 | acute respiratory illness | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000575 | epidemic start week | No | 0 |  |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000576 | epidemic peak week | No | 0 |  |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000637 | intent to be vaccinated | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000638 | intent to vaccinate | No | 0 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00000650 | intent to have a ward vaccinated | No | 1 | http://purl.obolibrary.org/obo/BFO_0000019 |  |
| http://purl.obolibrary.org/obo/APOLLO_SV_00001008 | TODO | No | 7 | http://purl.obolibrary.org/obo/BFO_0000001 |  |
