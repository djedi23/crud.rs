## Pretty Struct

Displays (json) structures and enums in a pretty way.

This crate is linked to the crud library. If I have time and motivation to generalize it, it can be an indenpendant crate.

### Example

```rust
use crud_pretty_struct::PrettyPrint;
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(color="green")]
    a: u32,
    #[pretty(skip_none)]
    b: Option<String>,
    #[pretty(formatter=crud_pretty_struct::formatters::bool_check_formatter)]
    c: bool,
    #[pretty(is_pretty)]
    d: OtherPrettyStruct
}
// Instanciate a `var` of type  `Foo`
println!("{}",var.pretty(true,None,None).expect("Can prettify var"));
```

### Field Options

###### `is_pretty`

the nested struct implements `PrettyPrint` and should be printed using it.

```rust
use crud_pretty_struct_derive::PrettyPrint;
#[derive(PrettyPrint)]
struct OtherPrettyStruct {}
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(is_pretty)]
    field: OtherPrettyStruct
}
```

###### `label`

custom label for this field
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(label="☀️ my field")]
    field: u32
}
```
###### `color`

custom color for this field. The avaiable colors are [Color].
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(color="red")]
    field: u32
}
```
###### `label_color`

custom color for the label of this field. The avaiable colors are [Color].
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(color="red")]
    field: u32
}
```
###### `skip`

skip the field. It won't be display.
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(skip)]
    field: u32
}
```
###### `skip_none`

skip the field  if the value is `None`. The type of the field should be an `Option<T>`.
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(skip_none)]
    field: Option<u32>
}
```
###### `formatter`

Custom value formatter for this field.

Some [formatters] are shipped in this crate.
```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(formatter=crud_pretty_struct::formatters::bool_check_formatter)]
    field: bool
}
```

Formatters should follow this signature:
```rust
type Formatter = dyn Fn(/*value:*/ &dyn ToString, /*colored:*/ bool) -> miette::Result<(String, bool)>;
```
Parameters:
- `value`: the value to format
- `colored`: when `true` the formatted value can be colored

Result:
- String: the formatted value
- bool: returns `true` if the value have colors. returns `false` if the value don't have colors then default color will be applied.

```rust
#[derive(PrettyPrint)]
struct Foo {
    #[pretty(formatter=|x, _| Ok((format!("{} kg", x.to_string()), false)))]
    field: f32
}
```

### Enum Option

Limitations on enums:
- unit variants are supported
- tuple variants with only 1 argument are supported

###### `color`

custom color for this variant avaiable colors are [Color].
```rust
#[derive(PrettyPrint)]
enum Foo {
    #[pretty(color="red")]
    Variant
}
```

###### `label`

custom label for this variant
```rust
#[derive(PrettyPrint)]
enum Foo {
    #[pretty(label="☀️ my field")]
    Variant
}
```


