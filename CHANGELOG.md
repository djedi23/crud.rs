<a name="unreleased"></a>
## [Unreleased]


<a name="v0.1.4"></a>
## [v0.1.4] - 2023-07-20
### Bug
- **crud-api-derive:** match enum user the field's name.
- **crud-api-endpoint:** scratch don't create dir.

### Feat
- **crud-api-derive:** rename #[api(format)] to #[api(table_format)]
- **crud-pretty-struct:** new crates crud-pretty-struct and crud-pretty-struct-derive. These crates allow to format struct as tree with custom label, colors, custom fields format, ...


<a name="v0.1.3"></a>
## [v0.1.3] - 2023-06-12
### Feat
- **build:** use scratch crate to manage endpoints.json build files.
- **crud-api:** type for input(s field can be a C-like enum. The variants are possible values in the cli.
- **crud-api:** enum payloads are now subcommands
- **crud-api:** ability to tranform the raw http document into the target type.
- **crud-api:** add `extra_action` attribute.
- **crud-api:** add `extra_header` attribute.
- **crud-api:** add a `no_auth` flag to the endpoints.
- **crud_auth_bearer:** Ability to `save_token`.


<a name="v0.1.2"></a>
## [v0.1.2] - 2023-04-07
### Chore
- update libs, cargo audit and v0.1.2
- update dependencies

### Feat
- **boolean:** Option<bool> type generates a flag. If the flag is present then it returns Some(true) else None. bool type force the use to set the arg to true or false.
- **upload:** upload streams as base64 in json payload.


<a name="v0.1.1"></a>
## [v0.1.1] - 2023-01-15

<a name="v0.1.0-p3"></a>
## [v0.1.0-p3] - 2022-12-31

<a name="v0.1.0-p2"></a>
## [v0.1.0-p2] - 2022-12-30

<a name="v0.1.0-p1"></a>
## [v0.1.0-p1] - 2022-12-30

<a name="v0.1.0"></a>
## v0.1.0 - 2022-12-30

[Unreleased]: https://github.com/djedi23/crud.rs/compare/v0.1.4...HEAD
[v0.1.4]: https://github.com/djedi23/crud.rs/compare/v0.1.3...v0.1.4
[v0.1.3]: https://github.com/djedi23/crud.rs/compare/v0.1.2...v0.1.3
[v0.1.2]: https://github.com/djedi23/crud.rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/djedi23/crud.rs/compare/v0.1.0-p3...v0.1.1
[v0.1.0-p3]: https://github.com/djedi23/crud.rs/compare/v0.1.0-p2...v0.1.0-p3
[v0.1.0-p2]: https://github.com/djedi23/crud.rs/compare/v0.1.0-p1...v0.1.0-p2
[v0.1.0-p1]: https://github.com/djedi23/crud.rs/compare/v0.1.0...v0.1.0-p1
