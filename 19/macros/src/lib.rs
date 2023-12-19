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

        fn R(part: &Part) -> bool {
            false
        }
        fn A(part: &Part) -> bool {
            true
        }
    };

    let mut reading_rules = true;
    let mut parts: Vec<proc_macro2::TokenStream> = Vec::new();
    for ln in std::fs::read_to_string(&filename).unwrap_or_else(|_| panic!("missing input file '{:?}'", filename)).lines() {
        if ln.is_empty() {
            reading_rules = false;
            out.extend(quote!(  ));
        } else if reading_rules {
            out.extend(workflow_to_rust_code(ln));
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

fn workflow_to_rust_code(workflow: &str) -> proc_macro2::TokenStream {
    let mut out = String::new();
    out.push_str("fn r#");
    let workflow = workflow.strip_suffix('}').unwrap();
    let i = workflow.chars().position(|c| c=='{').unwrap();
    out.push_str(&workflow[0..i]);
    out.push_str("(part: &Part) -> bool {\n");
    for x in workflow[i+1..].split(',') {
        if x.contains(':') {
            out.push_str("    if part.");
            let (cond, next) = x.split_once(':').unwrap();
            out.push_str(cond);
            out.push_str(" { return r#");
            out.push_str(next);
            out.push_str("(part); }\n")
        } else {
            out.push_str("    return ");
            out.push_str(x);
            out.push_str("(part);\n")
        }
    }
    out.push('}');
    //println!("{}", out);
    parse_str(&out).expect("failed to parse workflow")
}
