use darling::{FromDeriveInput, FromMeta};
use syn::Ident;

// Copy from cruds.rs/CrudsConfig
/// Information block for `#[derive(ApiRun)]`.
///
/// # Example
/// ```rust
/// # use crud_api::ApiRun;
/// # use crud_auth::CrudAuth;
/// # use crud_auth_no_auth::Auth;
/// # use miette::IntoDiagnostic;
/// # #[derive(ApiRun)]
/// #[api(infos(
///   base_url = "http://jsonplaceholder.typicode.com",
///   name = "jsonplaceholder",
///   qualifier = "com",
///   organisation = "typicode",
///   env_prefix = "JSONPLACEHOLDER"
/// ))]
/// # struct JSONPlaceHoder;
///
/// ```
#[derive(Debug, Default, FromMeta)]
#[allow(dead_code)]
pub struct ApiInformation {
  /// Name of the application. If omitted, the crate's name is used.
  /// This name appears in help and is used to generate the config's path.
  pub name: Option<String>,
  /// The base URL of the api. All the endpoints are relative to this URL.
  pub base_url: String,
  /// Short description of the application. It appears in the help.
  pub about: Option<String>,
  /// The version of the application. If omitted, the crate's version is used.
  pub version: Option<String>,
  /// The author of the application. If omitted, the crate's author is used.
  pub author: Option<String>,
  /// A qualifier to generate the configuration path. If omitted, an empty
  /// string is used. The qualifier can be the TLD of the application's
  /// url. Example: "com"
  pub qualifier: Option<String>,
  /// A organisation to generate the configuration path. If omitted, an empty
  /// string is used. The organisation can be the domain of the application's
  /// url. Example: "foobar" in "foobar.com".
  pub organisation: Option<String>,
  /// A prefix for environment variables.Some parameters can be
  /// passed/overried by environment variables (base-url, auth-token). The
  /// environment variables will be generate by prefixing this
  /// parameter. Example: When `env_prefix` is "_MYAPP_" the `base_url`
  /// parameter become `MYPAPP_BASE_URL`. If omitted, the `env_prefix` is
  /// "_APP_".
  pub env_prefix: Option<String>,
}

/// Attribute used by `#[derive(ApiRun)]`.
///
/// It declare a new cli application:
/// - create the cli
/// - handle the command
///
/// It can take parameters by `#[api(...)]`.
/// Only one `#[derive(ApiRun)]` should be present in your application.
#[derive(FromDeriveInput)]
#[darling(attributes(api))]
pub struct ApiRun {
  /// Information block of the application.
  ///
  /// # Example
  /// ```rust
  /// # use crud_api::ApiRun;
  /// # use crud_auth::CrudAuth;
  /// # use crud_auth_no_auth::Auth;
  /// # use miette::IntoDiagnostic;
  /// # #[derive(ApiRun)]
  /// #[api(infos(
  ///   base_url = "http://jsonplaceholder.typicode.com",
  ///   name = "jsonplaceholder",
  ///   qualifier = "com",
  ///   organisation = "typicode",
  ///   env_prefix = "JSONPLACEHOLDER"
  /// ))]
  /// # struct JSONPlaceHoder;
  ///
  /// ```
  #[darling(default)]
  pub infos: ApiInformation,
  /// Name of the struct derived by `ApiRun`.
  /// Used to implentent the `run` function.
  #[doc(hidden)]
  pub ident: Ident,
  //  attrs: Vec<syn::Attribute>,
}
