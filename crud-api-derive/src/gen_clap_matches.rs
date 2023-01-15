use crate::gen_clap_declarations::strip_var;
use crud_api_endpoint::{endpoints, Emap, Endpoint};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

#[rustfmt::skip::macros(quote)]
fn match_endpoint(
  ep: &Endpoint,
  arg_ident: &Ident,
  ids: &[(Ident, TokenStream)],
) -> proc_macro2::TokenStream {
  let (paylay_decl, payload) = if let Some(payload_struct) = &ep.payload_struct {
    let payload_type = Ident::new(payload_struct, Span::call_site());
    let error_context = format!("Can't read payload for '{payload_struct}'");
    (
      quote! {
          let payload = <#payload_type>::from_clap_matches(#arg_ident)
              .context(#error_context)?;
          log::trace!("Payload: {:#?}",payload);
      },
      quote! {Some(payload)},
    )
  } else {
    (quote! {}, quote! {None::<()>})
  };

  let (query_dec, query_args) = if let Some(query_struct) = &ep.query_struct {
    let query_type = Ident::new(query_struct, Span::call_site());
    let error_context = format!("Can't read query for '{query_struct}'");
    (
      quote! {
          let query = <#query_type>::from_clap_matches(#arg_ident)
              .context(#error_context)?;
          log::trace!("Query: {:?}",query);
      },
      quote! {Some(query)},
    )
  } else {
    (quote! {}, quote! {None::<()>})
  };

  let uri = &ep.route;
  let urif = format!("{{}}{uri}");
  let ids: TokenStream = ids
    .iter()
    .map(|(ident, _dec)| quote!(, #ident=#ident))
    .collect();
  let result = Ident::new(&ep.result_struct, Span::call_site());
  let result_type = if ep.result_multiple {
    quote! {Vec<#result>}
  } else {
    quote! {#result}
  };

  let output_format = if ep.cli_no_output {
    quote!(None)
  } else {
    quote!(crud_api::clap_match_output_format(#arg_ident))
  };

  let result_output = if ep.result_multiple {
    quote! {#result :: output_multiple(&result, #output_format )?;}
  } else {
    quote! {result.output(#output_format)?;}
  };
  let method = Ident::new(&ep.method, Span::call_site());
  let status = Ident::new(&ep.result_ok_status, Span::call_site());
  let ko_status: TokenStream = ep
    .result_ko_status
    .iter()
    .map(|s| {
      let status = Ident::new(&s.status, Span::call_site());
      let msg = &s.message;
      quote!{
	    h.insert(hyper::StatusCode::#status, #msg.into());
	}
    })
    .collect();
  let ko_status_map = if ko_status.is_empty() {
    quote!(&std::collections::HashMap::new())
  } else {
    quote!{
	&{
	    let mut h = std::collections::HashMap::new();
	    #ko_status
	    h
	}
    }
  };

  let query_and_print = if ep.result_is_stream {
    quote!(crud_api::http::HTTPApi::new(format!(#urif,base_url #ids),
				     hyper::Method::#method,
				     hyper::StatusCode::#status,
				     #ko_status_map,
				     &auth)
	   .stream(#payload,
		   #query_args,
		   #arg_ident.get_one::<String>("output_file").cloned()).await?;
    )
  } else {
    quote!(
        let result:#result_type =
	    crud_api::http::HTTPApi::new(format!(#urif,base_url #ids),
				      hyper::Method::#method,
				      hyper::StatusCode::#status,
				      #ko_status_map,
				      &auth)
	    .query(#payload,#query_args).await?;
        #result_output
    )
  };

  quote! {
      #query_dec
      #paylay_decl
      #query_and_print
    }
}

#[rustfmt::skip::macros(quote)]
fn argmatches_rec(
  endpoints_map: &Emap,
  last_match: Option<Ident>,
  ids: Vec<(Ident, TokenStream)>,
) -> Vec<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)> {
  endpoints_map
    .iter()
    .map(|(segment, node)| {
      if let Some(var) = strip_var(segment) {
        let var_ident = Ident::new(var, Span::call_site());
        let new_ids = if let Some(arg_ident) = &last_match {
          let vardec = quote!{
	      if !#arg_ident.contains_id(#var) { miette::bail!("<{}> is required",#var)};
	      let #var_ident = #arg_ident.get_one::<String>(#var).cloned().unwrap();
	  };
          let mut ids_mut = ids.clone();
          ids_mut.push((var_ident.to_owned(), vardec));
          ids_mut
        } else {
          ids.clone()
        };
        let argmatches = argmatches_rec(&node.route, last_match.to_owned(), new_ids.clone());
        let argmatches: Vec<proc_macro2::TokenStream> =
          argmatches.iter().map(|(f, _)| f.to_owned()).collect();
        let var_do_query = if let Some(arg_ident) = &last_match {
          let endpoints = &node.endpoint;
          if endpoints.is_empty() {
            None
          } else {
            let do_query_var: TokenStream = endpoints
              .iter()
              .map(|ep| match_endpoint(ep, arg_ident, &new_ids))
              .collect();
            Some(quote!{
		  if #arg_ident.contains_id(#var) {
		      let #var_ident = #arg_ident.get_one::<String>(#var).cloned().unwrap();
		      #do_query_var
		  }
	      })
          }
        } else {
          None
        };
        (quote!(#(#argmatches)*), var_do_query)
      } else {
        let arg_ident = Ident::new(&format!("{segment}_arg"), Span::call_site());
        let argmatches = argmatches_rec(&node.route, Some(arg_ident.to_owned()), ids.to_owned());
        let do_query_when_id_matched: Vec<proc_macro2::TokenStream> = argmatches
          .iter()
          .filter(|(_, v)| v.is_some())
          .map(|(_, v)| {
            if let Some(v) = v {
              quote!(#v else)
            } else {
              quote!()
            }
          })
          .collect();
        let subcommands_matches: Vec<proc_macro2::TokenStream> =
          argmatches.iter().map(|(f, _)| f.to_owned()).collect();
        let endpoints = &node.endpoint;
        let do_query_when_no_ids: TokenStream = endpoints
          .iter()
          .map(|ep| match_endpoint(ep, &arg_ident, &ids))
          .collect();

        let ids_dec: TokenStream = ids.iter().map(|(_, dec)| dec.to_owned()).collect();

        let submatches = if subcommands_matches.is_empty() {
          quote!(
		#ids_dec
		#do_query_when_no_ids
	    )
        } else {
          // else: we have subcommands.
          quote!{
	      match #arg_ident.subcommand() {
		  #(#subcommands_matches)*
		  Some((_,_))=> commands.print_help().into_diagnostic()?,
		  None => {
		      #ids_dec
		      #(#do_query_when_id_matched)*
		      { #do_query_when_no_ids }
		  },
	      }
	  }
        };
        (
          quote!{
	      Some((#segment,#arg_ident)) => {
		  #submatches
	      },
	  },
          None,
        )
      }
    })
    .collect::<Vec<(proc_macro2::TokenStream, Option<proc_macro2::TokenStream>)>>()
}

pub(crate) fn argmatches() -> proc_macro2::TokenStream {
  let eps = endpoints();
  let matches = argmatches_rec(&eps, None, vec![]);
  let matches: proc_macro2::TokenStream = matches.iter().map(|(f, _)| f.to_owned()).collect();
  //  println!("{}", matches);

  quote! {#matches}
}
