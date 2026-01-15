extern crate proc_macro;

use proc_macro::TokenStream;

use syn::{parse, parse_macro_input, DeriveInput, ItemImpl};

use args::{Casts, Flag, Targets};
use gen_caster::generate_caster;

mod args;
mod gen_caster;
mod item_impl;
mod item_type;

#[proc_macro_attribute]
pub fn cast_to(args: TokenStream, input: TokenStream) -> TokenStream {
    match parse::<Targets>(args) {
        Ok(Targets { flags, paths }) => {
            if paths.is_empty() {
                item_impl::process(&flags, parse_macro_input!(input as ItemImpl))
            } else {
                item_type::process(&flags, paths, parse_macro_input!(input as DeriveInput))
            }
        }
        Err(err) => vec![err.to_compile_error(), input.into()]
            .into_iter()
            .collect(),
    }
    .into()
}

#[proc_macro]
pub fn castable_to(input: TokenStream) -> TokenStream {
    let Casts {
        ty,
        targets: Targets { flags, paths },
    } = parse_macro_input!(input);

    paths
        .iter()
        .map(|t| generate_caster(&ty, t, flags.contains(&Flag::Sync)))
        .collect::<proc_macro2::TokenStream>()
        .into()
}
