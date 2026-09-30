# COB Alignment Report for PPO

In the table below, "aligned classes" are classes that have at least one ancestor that is a term in COB.

| Class Set | Number of Classes | Number of Aligned Classes | Alignment % |
| ----- | ----- | ----- | ----- |
| All classes (including imports) | 575 | 269 | 46.78% |
| Classes in PPO namespace | 400 | 103 | 25.75% |

PPO has 25 unaligned roots. An unaligned root is the highest-level in-namespace term without a COB ancestor. To align PPO with COB, these terms should be moved under COB terms or added to COB.

The table below contains all unaligned roots in PPO. The column 'Preferred Root?' indicates whether a term has an `IAO:0000700` annotation marking it as a preferred root in the ontology.

| Root ID | Root Label | Preferred Root? | Subclasses | Lowest BFO Ancestor ID | Suggested Replacement |
| ----- | ----- | ----- | ----- | ----- | ----- |
| PPO:0001025 | floral structure | No | 0 |  |  |
| PPO:0001043 | ripening fruit | No | 0 |  |  |
| PPO:0001044 | unripe fruit | No | 0 |  |  |
| PPO:0001050 | ripening megasporangiate strobilus | No | 0 |  |  |
| PPO:0001052 | ripe megasporangiate strobilus | No | 0 |  |  |
| PPO:0001053 | portion of a plant | No | 0 |  |  |
| PPO:0002000 | plant phenological trait | No | 272 | BFO:0000020 | characteristic |
| PPO:0002045 | ripening fruit presence | No | 0 |  |  |
| PPO:0002046 | unripe fruit presence | No | 0 |  |  |
| PPO:0002055 | ripening megasporangiate strobilus presence | No | 0 |  |  |
| PPO:0002057 | ripe megasporangiate strobilus presence | No | 0 |  |  |
| PPO:0002343 | ripening fruit present | No | 0 |  |  |
| PPO:0002344 | unripe fruit present | No | 0 |  |  |
| PPO:0002353 | ripening megasporangiate strobilus present | No | 0 |  |  |
| PPO:0002355 | ripe megasporangiate strobilus present | No | 0 |  |  |
| PPO:0002642 | ripening fruit absent | No | 0 |  |  |
| PPO:0002643 | unripe fruit absent | No | 0 |  |  |
| PPO:0002652 | ripening megasporangiate strobilus absent | No | 0 |  |  |
| PPO:0002654 | ripe megasporangiate strobilus absent | No | 0 |  |  |
| PPO:0007000 | plant growth cycle | No | 0 |  |  |
| PPO:0007001 | initial growth cycle | No | 0 |  |  |
| PPO:0007002 | later growth cycle | No | 0 |  |  |
| PPO:0007003 | plant regreening process | No | 0 |  |  |
| PPO:0007012 | fruit unripe stage | No | 0 |  |  |
| PPO:0007019 | megasporangiate strobilus ripe stage | No | 0 |  |  |
