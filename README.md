# block-entity-finder
Tool to find chunks with the most block entities

## Example

```
$ block-entity-finder ~/.minecraft/saves/New\ World/ 0
Chunk (-23, 29): 6 block entities
         1: minecraft:chest at (-358, 26, 470)
         2: minecraft:chest at (-355, 51, 466)
         3: minecraft:chest at (-356, 51, 467)
         4: minecraft:mob_spawner at (-357, 51, 464)
         5: minecraft:chest at (-355, 26, 469)
         6: minecraft:mob_spawner at (-356, 26, 471)
Chunk (56, 26): 4 block entities
         1: minecraft:mob_spawner at (902, 41, 417)
         2: minecraft:chest at (900, 41, 418)
         3: minecraft:chest at (904, 29, 431)
         4: minecraft:chest at (900, 41, 416)
Chunk (19, 5): 4 block entities
         1: minecraft:chest at (314, 53, 92)
         2: minecraft:chest at (316, 53, 90)
         3: minecraft:chest at (314, 53, 88)
         4: minecraft:chest at (312, 53, 90)
Chunk (2, 69): 4 block entities
         1: minecraft:chest at (42, 53, 1116)
         2: minecraft:chest at (44, 53, 1114)
         3: minecraft:chest at (40, 53, 1114)
         4: minecraft:chest at (42, 53, 1112)
Chunk (56, 9): 3 block entities
         1: minecraft:chest at (906, 36, 154)
         2: minecraft:mob_spawner at (908, 36, 152)
         3: minecraft:chest at (905, 36, 152)
Chunk (55, 45): 3 block entities
         1: minecraft:mob_spawner at (890, 43, 727)
         2: minecraft:chest at (889, 43, 729)
         3: minecraft:chest at (888, 43, 727)
Chunk (51, 29): 3 block entities
         1: minecraft:chest at (820, 13, 475)
         2: minecraft:mob_spawner at (819, 13, 472)
         3: minecraft:chest at (821, 13, 471)
Chunk (51, 6): 3 block entities
         1: minecraft:mob_spawner at (829, 25, 109)
         2: minecraft:chest at (831, 25, 110)
         3: minecraft:chest at (828, 25, 107)
Chunk (49, 40): 3 block entities
         1: minecraft:chest at (790, 18, 644)
         2: minecraft:chest at (789, 18, 642)
         3: minecraft:mob_spawner at (788, 18, 644)
Chunk (48, 84): 3 block entities
         1: minecraft:mob_spawner at (782, 13, 1346)
         2: minecraft:chest at (783, 13, 1344)
         3: minecraft:chest at (781, 13, 1348)
```

## Building

`block-entity-finder` is written in [Rust](https://rust-lang.org) so you need a Rust compiler to
build it.

```
$ git clone https://github.com/andreasots/block-entity-finder
$ cd block-entity-finder
$ cargo build --release
$ target/release/block-entity-finder --help
```
