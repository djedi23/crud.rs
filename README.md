# CRUD

A rust workspace to create command line interfaces (CLI) to HTTP API.

For example, you can create a CLI for jsonplaceholder with this sniplet:

```rust
#[derive(Debug, Crud, Deserialize, Serialize, Default)]
struct Posts {
  #[crud(id, no_short)]
  id: u32,
  #[crud(help = "post author's id")]
  #[serde(rename = "userId")]
  user_id: u32,
  #[crud(help = "title of the post")]
  #[crud(no_short)]
  title: String,
  #[crud(help = "body of the post")]
  body: String,
}
```

After compiling (and some boitlerplates; uses, main function, ...), you'll obtain:

```shell
$ jsonplaceholder posts -h
Posts listing

Usage: jsonplaceholder posts [OPTIONS] [id] [COMMAND]

Commands:
  create    Posts creation
  delete    Posts deletion
  update    Posts update
  replace   Posts replacement
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [id]

Options:
  -h, --help  Print help information (use `--help` for more detail)
```
```shell
$ jsonplaceholder posts create -h
Posts creation

Usage: jsonplaceholder posts create [OPTIONS] --user_id <user_id> --title <title> --body <body>

Options:
  -h, --help  Print help information (use `--help` for more detail)

Payload:
  -u, --user_id <user_id>  post author's id
      --title <title>      title of the post
  -b, --body <body>        body of the post

```

The complete code of this example is at [`./crud/examples/jsonplaceholder.rs`](./crud/examples/jsonplaceholder.rs)


## Sub Crates


- [crud](./crud): Entrypoint to describes REST API.
- [crud-api](./crud-api): Describes general API and utilities.
- [crud-derive](./crud-derive): `Crud` derive crates.
- [crud-api-derive](./crud-api-derive): `Api` derive crates.
- [crud-api-endpoint](./crud-api-endpoint): Endpoint crates. Used by `crud-api` and `crud` crates.
- [crud-auth](./crud-auth): `CrudAuth` trait.
- [crud-auth-bearer](./crud-auth-bearer): Implementation of `CrudAuth` trait for bearer auth.
- [crud-auth-no-auth](./crud-auth-no-auth): Implementation of `CrudAuth` trait for no authentification.
- [crud-pretty-struct](./crud-pretty-struct): Custom pretty printer for structs.
- [crud-tidy-viewer](./crud-tidy-viewer): array pretty printer.
