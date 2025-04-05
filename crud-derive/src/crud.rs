use crud_api_endpoint::{
  store_endpoint, table_impl, ApiField, ApiVariant, EndpointBuilder, FieldFormat,
};
use darling::{
  ast::{Data, Fields},
  FromDeriveInput, FromField, FromMeta,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Attribute, DeriveInput, Ident, MetaList, Type};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(crud), forward_attrs(serde))]
pub struct CrudField {
  ident: Option<Ident>,
  ty: Type,
  attrs: Vec<Attribute>,
  /// Mark this field as `id`
  #[darling(default)]
  id: Option<bool>,
  /// Long name of the option
  long: Option<String>,
  /// Short name of the option
  short: Option<char>,
  /// Don't generate a short option
  #[darling(default)]
  no_short: bool,
  /// Category of the option
  heading: Option<String>,
  /// Short help string
  help: Option<String>,
  /// Long help text
  long_help: Option<String>,
  /// the field won't appears when display as the table
  #[darling(default)]
  table_skip: bool,
  /// Format of the field
  pub table_format: Option<FieldFormat>,
}

impl From<CrudField> for ApiField {
  fn from(cf: CrudField) -> Self {
    Self {
      ident: cf.ident,
      ty: cf.ty,
      table_skip: cf.table_skip,
      table_format: cf.table_format,
    }
  }
}

#[derive(FromDeriveInput, Debug)]
#[darling(attributes(crud), forward_attrs(derive))]
struct Crud {
  ident: Ident,
  pub data: Data<ApiVariant, CrudField>,
  pub attrs: Vec<syn::Attribute>,

  /// Endpoint route prefix. example: `route="/myroute"`
  route: Option<String>,
  /// Nested link to this endpoind. example: `nested(route = "/another_endpoint/{id}/here"))`
  #[darling(multiple)]
  nested: Vec<CrudNested>,
  /// Parameter struct that is passed in the query string
  parameters: Option<String>,
  /// Help string
  help: Option<String>,
}

#[derive(FromMeta, Debug)]
struct CrudNested {
  route: String,
}

#[rustfmt::skip::macros(quote)]
pub(crate) fn crud_expension(ast: &DeriveInput) -> TokenStream {
  let crud: Crud = Crud::from_derive_input(ast).unwrap();
  let route = if let Some(route) = crud.route {
    route
  } else {
    format!("/{}", crud.ident.to_string().to_lowercase())
  };

  let is_pretty = crud.attrs.iter().any(|Attribute { meta, .. }| match meta {
    syn::Meta::List(MetaList { tokens, .. }) => tokens
      .clone()
      .into_iter()
      .any(|ident| ident.to_string() == "PrettyPrint"),
    _ => false,
  });

  let list = EndpointBuilder::default()
    .route(route.to_owned())
    .cli_route(route.to_owned())
    .result_struct(crud.ident.to_string())
    .result_multiple(true)
    .query_struct(crud.parameters.to_owned())
    .cli_help(if let Some(help) = crud.help {
      help
    } else {
      crud.ident.to_string() + " listing"
    })
    .build()
    .unwrap();
  store_endpoint(list);

  for nested_route in &crud.nested {
    let nested_endpoint = EndpointBuilder::default()
      .route(nested_route.route.to_owned())
      .cli_route(nested_route.route.to_owned())
      .result_struct(crud.ident.to_string())
      .result_multiple(true)
      .query_struct(crud.parameters.to_owned())
      .build()
      .unwrap();
    store_endpoint(nested_endpoint);
  }

  let base_arg = route.to_owned() + "/{id}";

  let read = EndpointBuilder::default()
    .route(base_arg.to_owned())
    .cli_route(base_arg.to_owned())
    .result_struct(crud.ident.to_string())
    .build()
    .unwrap();
  store_endpoint(read);

  let delete = EndpointBuilder::default()
    .route(base_arg.to_owned())
    .cli_route(base_arg.to_owned() + "/delete")
    .method("DELETE")
    .result_struct("EmptyResponse")
    .cli_help(crud.ident.to_string() + " deletion")
    .build()
    .unwrap();
  store_endpoint(delete);

  let create = EndpointBuilder::default()
    .route(route.to_owned())
    .cli_route(route + "/create")
    .method("POST")
    .result_ok_status("CREATED")
    .result_struct(crud.ident.to_string())
    .payload_struct(suffix_struct_ident(&crud.ident, "CreatePayload").to_string())
    .cli_help(crud.ident.to_string() + " creation")
    .build()
    .unwrap();
  store_endpoint(create);

  let update = EndpointBuilder::default()
    .route(base_arg.to_owned())
    .cli_route(base_arg.to_owned() + "/update")
    .method("PATCH")
    .result_struct(crud.ident.to_string())
    .payload_struct(suffix_struct_ident(&crud.ident, "UpdatePayload").to_string())
    .cli_help(crud.ident.to_string() + " update")
    .cli_long_help(crud.ident.to_string() + " update. All fields are optional.")
    .build()
    .unwrap();
  store_endpoint(update);

  let replace = EndpointBuilder::default()
    .route(base_arg.to_owned())
    .cli_route(base_arg + "/replace")
    .method("PUT")
    .result_struct(crud.ident.to_string())
    .payload_struct(suffix_struct_ident(&crud.ident, "ReplacePayload").to_string())
    .cli_help(crud.ident.to_string() + " replacement")
    .cli_long_help(crud.ident.to_string() + " replacement. All fields are required.")
    .build()
    .unwrap();
  store_endpoint(replace);

  let create_payload = create_payload(&crud.ident, &crud.data);
  let update_payload = update_payload(&crud.ident, &crud.data);
  let replace_payload = replace_payload(&crud.ident, &crud.data);
  let table = table_impl(&crud.ident, &crud.data, is_pretty);
  let ident = crud.ident;
  let out = quote! {
      #create_payload
      #update_payload
      #replace_payload
      #table
      impl TryFrom<crud_api::DummyTryFrom> for #ident {
	  type Error = String;
	  fn try_from(_value: crud_api::DummyTryFrom) -> std::result::Result<Self, Self::Error> {
	      Err(String::new())
	  }
      }
  };
  #[cfg(feature = "dump-derives")]
  println!("{}", out);
  out
}

fn suffix_struct_ident(struct_ident: &Ident, suffix: &str) -> Ident {
  Ident::new(&(struct_ident.to_string() + suffix), Span::call_site())
}

fn field_annotations(field: &CrudField) -> TokenStream {
  let long = if let Some(long) = &field.long {
    quote!(#[api(long=#long)])
  } else {
    quote!()
  };
  let short = if let Some(short) = &field.short {
    quote!(#[api(short=#short)])
  } else {
    quote!()
  };
  let no_short = if field.no_short {
    quote!(#[api(no_short)])
  } else {
    quote!()
  };

  let help = if let Some(help) = &field.help {
    quote!(#[api(help=#help)])
  } else {
    quote!()
  };
  let long_help = if let Some(long_help) = &field.long_help {
    quote!(#[api(long_help=#long_help)])
  } else {
    quote!()
  };
  let heading = if let Some(heading) = &field.heading {
    quote!(#[api(heading=#heading)])
  } else {
    quote!()
  };
  quote!(#long #short #no_short #help #long_help #heading)
}

#[rustfmt::skip::macros(quote)]
fn create_payload(struct_ident: &Ident, data: &Data<ApiVariant, CrudField>) -> TokenStream {
  let payload_name = suffix_struct_ident(struct_ident, "CreatePayload");
  let quoted_fields = match data {
    Data::Enum(_) => {
      todo!("Générer les entetes et les données pour chaque variant");
      //      (vec![], quote!())
    }
    Data::Struct(Fields { fields, .. }) => fields
      .iter()
      .filter(|f| if let Some(id) = f.id { !id } else { true })
      .map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let typef = &field.ty;
        let annotations = field_annotations(field);
        let attrs = &field.attrs;
        quote!{
	      #(#attrs)*
	      #annotations
	      #ident: #typef,}
      }),
  };

  quote!{
      #[derive(serde::Serialize,serde::Deserialize,Default,Debug,ApiInput)]
      struct #payload_name {
	  #(#quoted_fields)*
	}
    }
}

#[rustfmt::skip::macros(quote)]
fn replace_payload(struct_ident: &Ident, data: &Data<ApiVariant, CrudField>) -> TokenStream {
  let payload_name = suffix_struct_ident(struct_ident, "ReplacePayload");
  let quoted_fields = match data {
    Data::Enum(_) => {
      todo!("Générer les entetes et les données pour chaque variant");
      //      (vec![], quote!())
    }
    Data::Struct(Fields { fields, .. }) => fields.iter().map(|field| {
      let ident = field.ident.as_ref().unwrap();
      let typef = &field.ty;
      let annotations = field_annotations(field);
      let attrs = &field.attrs;
      quote!{
	    #(#attrs)*
	    #annotations
	    #ident: #typef,}
    }),
  };

  quote!{
      #[derive(serde::Serialize,serde::Deserialize,Default,Debug,ApiInput)]
      struct #payload_name {
	  #(#quoted_fields)*
	}
    }
}

#[rustfmt::skip::macros(quote)]
fn update_payload(struct_ident: &Ident, data: &Data<ApiVariant, CrudField>) -> TokenStream {
  let payload_name = suffix_struct_ident(struct_ident, "UpdatePayload");
  let quoted_fields = match data {
    Data::Enum(_) => {
      todo!("Générer les entetes et les données pour chaque variant");
      //      (vec![], quote!())
    }
    Data::Struct(Fields { fields, .. }) => fields
      .iter()
      .filter(|f| if let Some(id) = f.id { !id } else { true })
      .map(|field| {
        let ident = field.ident.as_ref().unwrap();
        let typef = &field.ty;
        let annotations = field_annotations(field);
        let attrs = &field.attrs;
        quote!{
	    #(#attrs)*
	    #annotations
	    #[serde(skip_serializing_if = "Option::is_none")]
	    #ident: Option<#typef>,}
      }),
  };

  quote!{
      #[derive(serde::Serialize,serde::Deserialize,Default,Debug,crud_api::ApiInput)]
      struct #payload_name {
	  #(#quoted_fields)*
	}
    }
}
