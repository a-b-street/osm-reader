# osm-reader

This crate reads an OpenStreetMap XML or PBF file, letting the caller ignore the details about the format. It returns all nodes, ways, and relations in a format convenient for further stream processing. The heavy lifting is done by [roxmltree](https://crates.io/crates/roxmltree) and [osmpbf](https://crates.io/crates/osmpbf).

Status: under development. Use [osmio](https://crates.io/crates/osmio) for something more feature-rich and mature.

TODO before a crates.io release:

- [ ] Use an iterator (and the lower-level xmlparser approach from [osm2streets](https://github.com/a-b-street/osm2streets/blob/main/streets_reader/src/osm_reader/reader.rs)) to minimize memory
- [ ] Other TODOs in the code
- [ ] Tests, docs, examples
