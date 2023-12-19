use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_str,parse_macro_input,LitStr};
use home_dir::HomeDirExt;

#[proc_macro]
pub fn load(filename: TokenStream) -> TokenStream {
    let filename = parse_macro_input!(filename as LitStr).value();
    let filename = std::path::Path::new(&filename).expand_home().unwrap();

    let mut out = quote! {
        type CategoryType = u32;

        #[derive(Debug)]
        struct Part {
            x: CategoryType,
            m: CategoryType,
            a: CategoryType,
            s: CategoryType,
        }
    };

    let mut reading_rules = true;
    let mut parts: Vec<proc_macro2::TokenStream> = Vec::new();
    for ln in std::fs::read_to_string(&filename).unwrap_or_else(|_| panic!("missing input file '{:?}'", filename)).lines() {
        if ln.is_empty() {
            reading_rules = false;
            out.extend(quote!(  ));
        } else if reading_rules {
        } else {
            let ln = ln.replace('=', ":");
            parts.push(parse_str(&ln).expect("failed to parse part"));
        }
    }

    let parts_len = parts.len();
    out.extend(quote!(
        const PARTS: [Part; #parts_len] = [
            #(Part #parts,)*
        ];
    ));

    out.into()
}
