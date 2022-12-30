## Crud tidy viewer

Pretty printer for arrays.

Some parts of the code is copied and refactored from [Tidy-viewer](https://github.com/alexhallam/tv) (released under public domain)

### Examples

```rust
use crud_tidy_viewer::{display_table, TableConfig};
  let rdr = vec![
    vec!["a".to_string(), "b".to_string()],
    vec!["1".to_string(), "b".to_string()],
    vec!["4.1453".to_string(), "c".to_string()],
    vec!["2.4".to_string(), "f".to_string()],
    vec!["5".to_string(), "e".to_string()],
  ];
  display_table(&rdr, TableConfig::default());
```
