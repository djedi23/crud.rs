use crate::input::{field_quote, ApiInputField};
use crud_api_endpoint::{arg_config, endpoints, ApiInputConfig, Emap, Endpoint, VecStringWrapper};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn is_var(segment: &str) -> bool {
  segment.starts_with('{') && segment.ends_with('}')
}

pub(crate) fn strip_var(segment: &str) -> Option<&str> {
  if is_var(segment) {
    Some(
      segment
        .strip_prefix('{')
        .unwrap()
        .strip_suffix('}')
        .unwrap(),
    )
  } else {
    None
  }
}

/// Generate the '--format' argument if needed.
fn output_format(ep: &[Endpoint]) -> proc_macro2::TokenStream {
  if ep
    .iter()
    .any(|ep| !ep.result_is_stream && !ep.cli_no_output)
  {
    let mut format_list = ep
      .iter()
      .filter(|ep| !ep.cli_no_output)
      .flat_map(|ep| {
        ep.cli_output_formats
          .as_ref()
          .unwrap_or(&VecStringWrapper {
            v: vec![],
            c: vec![],
          })
          .v
          .to_owned()
      })
      .collect::<Vec<String>>();
    format_list.sort();
    format_list.dedup();
    let formats = if format_list.is_empty() {
      quote!(None)
    } else {
      quote!(Some(&[#(#format_list),*]))
    };

    let output_arg_config = arg_config(
      "output_format",
      &ep
        .iter()
        .flat_map(|ep| {
          if !ep.result_is_stream {
            ep.config.to_owned()
          } else {
            vec![]
          }
        })
        .collect::<Vec<ApiInputConfig>>(),
    );
    let long = output_arg_config.long.unwrap();
    let short = output_arg_config.short.unwrap();
    let heading = output_arg_config.heading.unwrap();

    quote!(let command = crud_api::clap_output_format_decl(command, #formats,
								#long, #short,#heading); )
  } else {
    quote! {}
  }
}

#[rustfmt::skip::macros(quote)]
fn subcommand_rec(endpoints_map: &Emap) -> (proc_macro2::TokenStream, bool) {
  let mut output_file_flag = true;
  (
    endpoints_map
      .iter()
      .map(|(segment, node)| {
        let (subcommands, output_file_flag_) = subcommand_rec(&node.route);
        output_file_flag = output_file_flag_;
        let ep = &node.endpoint;
        if let Some(var) = strip_var(segment) {
          let output_file = if output_file_flag && ep.iter().any(|ep| ep.result_is_stream) {
            let field: ApiInputField = arg_config(
              "output_file",
              &ep
                .iter()
                .flat_map(|ep| {
                  if ep.result_is_stream {
                    ep.config.to_owned()
                  } else {
                    vec![]
                  }
                })
                .collect::<Vec<ApiInputConfig>>(),
            )
            .into();
            output_file_flag = false;
            let arg = field_quote(&field, None, None);
            quote!{.arg(#arg)}
          } else {
            quote!()
          };

          let output_format = if ep.iter().any(|ep| ep.cli_force_output_format) {
            output_format(ep)
          } else {
            quote!{}
          };

          let query_args: Vec<TokenStream> = ep
            .iter()
            .filter(|ep| ep.query_struct.is_some())
            .map(|ep| {
              let query_type = Ident::new(ep.query_struct.as_ref().unwrap(), Span::call_site());
              quote!{let command = <#query_type>::clap(command,None);}
            })
            .collect();

          quote!(.arg(clap::Arg::new(#var))#output_file;
		 #output_format
		 #(#query_args)*
		 let command = command #subcommands)
        } else {
          let args: Vec<TokenStream> = ep
            .iter()
            .filter(|ep| ep.payload_struct.is_some())
            .map(|ep| {
              let payload_type = Ident::new(ep.payload_struct.as_ref().unwrap(), Span::call_site());
              quote! {let command = <#payload_type>::clap(command,None);}
            })
            .collect();
          let query_args: Vec<TokenStream> = ep
            .iter()
            .filter(|ep| ep.query_struct.is_some())
            .map(|ep| {
              let query_type = Ident::new(ep.query_struct.as_ref().unwrap(), Span::call_site());
              quote!{let command = <#query_type>::clap(command,None);}
            })
            .collect();

          let helps: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_help.is_some())
            .map(|ep| {
              let help = ep.cli_help.as_ref().unwrap();
              quote!{.about(#help)}
            })
            .collect();

          let long_helps: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_long_help.is_some())
            .map(|ep| {
              let help = ep.cli_long_help.as_ref().unwrap();
              quote!{.long_about(#help)}
            })
            .collect();

          let visible_aliases: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_visible_aliases.is_some())
            .map(|ep| {
              let visible_aliases = &ep.cli_visible_aliases.as_ref().unwrap().v;
              quote!{.visible_aliases([#(#visible_aliases ,)*])}
            })
            .collect();

          let long_flag_aliases: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_long_flag_aliases.is_some())
            .map(|ep| {
              let long_flag_aliases = &ep.cli_long_flag_aliases.as_ref().unwrap().v;
              quote!{.long_flag_aliases([#(#long_flag_aliases ,)*])}
            })
            .collect();

          let aliases: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_aliases.is_some())
            .map(|ep| {
              let aliases = &ep.cli_aliases.as_ref().unwrap().v;
              quote!{.aliases([#(#aliases ,)*])}
            })
            .collect();

          let short_flag_aliases: TokenStream = ep
            .iter()
            .filter(|ep| ep.cli_short_flag_aliases.is_some())
            .map(|ep| {
              let short_flag_aliases = &ep.cli_short_flag_aliases.as_ref().unwrap().c;
              quote!{.short_flag_aliases([#(#short_flag_aliases ,)*])}
            })
            .collect();

          let output_stream = if output_file_flag && ep.iter().any(|ep| ep.result_is_stream) {
            let field: ApiInputField = arg_config(
              "output_file",
              &ep
                .iter()
                .flat_map(|ep| {
                  if ep.result_is_stream {
                    ep.config.to_owned()
                  } else {
                    vec![]
                  }
                })
                .collect::<Vec<ApiInputConfig>>(),
            )
            .into();
            output_file_flag = false;
            let arg = field_quote(&field, None, None);
            quote!{ let command = command.arg(#arg);  /* #segment */}
          } else {
            quote!( )
          };

          let output_format = output_format(ep);

          if subcommands.is_empty() && args.is_empty() {
            quote! {.subcommand({let command = clap::Command::new(#segment)
				 .subcommand_precedence_over_arg(true)
				 #helps #long_helps
				 #visible_aliases #long_flag_aliases #aliases #short_flag_aliases;
				 #(#query_args)*
				 #output_format
				 #output_stream
				 command})}
          } else {
            let subcommands = if subcommands.is_empty() {
              quote!{}
            } else {
              quote!{let command = command #subcommands;}
            };

            quote! {.subcommand({
		let command = clap::Command::new(#segment)
		    .subcommand_precedence_over_arg(true)
		    #helps #long_helps
		#visible_aliases #long_flag_aliases #aliases #short_flag_aliases;
		#(#args)*
		#(#query_args)*
		#output_format
		#output_stream
		#subcommands
		command
	    })}
          }
        }
      })
      .collect::<proc_macro2::TokenStream>(),
    output_file_flag,
  )
}

pub(crate) fn subcommands() -> proc_macro2::TokenStream {
  let eps = endpoints();
  let (subcommands, _) = subcommand_rec(&eps);

  //  println!("{}", subcommands);
  quote! {commands = commands #subcommands ;}
}

#[cfg(test)]
mod tests {
  use crate::gen_clap_declarations::{is_var, strip_var, subcommand_rec};
  use crud_api_endpoint::{Emap, EpNode};
  use std::collections::HashMap;

  #[test]
  fn test_is_var() {
    assert_eq!(is_var("var"), false);
    assert_eq!(is_var("{var}"), true);
  }
  #[test]
  fn test_strip_var() {
    assert_eq!(strip_var("var"), None);
    assert_eq!(strip_var("{var}"), Some("var"));
  }

  #[test]
  fn test_subcommand_rec() {
    let mut endpoints_map: Emap = HashMap::new();
    let (t, o) = subcommand_rec(&endpoints_map);
    assert_eq!(t.to_string(), "");
    assert_eq!(o, true);

    endpoints_map.insert(
      "route".into(),
      EpNode {
        endpoint: vec![],
        route: HashMap::new(),
      },
    );

    let (t, o) = subcommand_rec(&endpoints_map);
    assert_eq!(
      t.to_string(),
      ". subcommand ({ let command = clap :: Command :: new (\"route\") . subcommand_precedence_over_arg (true) ; command })"
    );
    assert_eq!(o, true);
  }
}
