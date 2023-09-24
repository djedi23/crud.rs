//! # CRUD
//!
//! This crate provides a framework to generate an executable to manipulate your REST HTTP API from CLI.
//!
//! Have a look to the examples directory.
//!
//! ### Options
//!
//! #### Crud Options
//!
//! Per endpoint options.
//!
//! * **route** : route prefix. `route="/myroute"`
//! * **nested**: Nested link to this endpoind. example: `nested(route = "/another_endpoint/{id}/here"))`
//! * **parameters**: Parameter struct that is passed in the query string
//! * **help**: Help string
//!
//! #### Field Options
//!
//! * **id**: Mark this field as `id`
//! * **long**: Long name of the option
//! * **short**: Short name of the option
//! * **no_short**: Don't generate a short option
//! * **heading**: Category of the option
//! * **help**: Short help string
//! * **long_help**: Long help text
//! * **table_skip**: THE field won't appears when display as the table
//!
//!
//! ### Runtime Settings
//!
//! File `settings.toml`
//!
//! | option     | description          |                            |
//! |------------|----------------------|----------------------------|
//! | base_url   | Base url of the api  |                            |
//! | auth_token | token send as bearer | read by `crud-auth-bearer` |
//!
//! #### Profiles
//!
//! In `settings.toml`, you can define multiple profiles:
//! ```ignore
//! [profile.p1]
//! base_url="..."
//! uth_token="..."
//! [profile.p2]
//! base_url="..."
//! uth_token="..."
//! ```
//!
//! You call the profiles with the `--profile` argument.

extern crate crud_derive;
#[doc(hidden)]
pub use crud_derive::*;
