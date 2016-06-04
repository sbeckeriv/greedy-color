# ruru greedy-color

1. `cargo build --release` in main
2. `bundle exec ruby test.rb`

note you might need to install ruby with `--enable-shared`

# greedy-color

Using an adjacency list in a hash generate color numbers so that no two
colors are next to each other. Use the greedy algorithm because its easy.

```
3234 nodes
Rehearsal -----------------------------------------
ruby: 197.880000   1.280000 199.160000 (199.374137)
rust:  23.100000   1.380000  24.480000 ( 24.482296)
------------------------------ total: 223.640000sec

            user     system      total        real
ruby: 184.970000   0.550000 185.520000 (185.632575)
rust:  22.110000   1.280000  23.390000 ( 23.387126)
```
