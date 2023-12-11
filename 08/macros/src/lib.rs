extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;

use regex::Regex;
use once_cell::sync::Lazy;

static RE_NODE_LINE: Lazy::<Regex> = Lazy::new(|| Regex::new(r"^(?<name>[A-Z1-9]+)\s*=\s*\((?<left>[A-Z1-9]+),\s*(?<right>[A-Z1-9]+)\)$").unwrap());


#[proc_macro]
pub fn definitions(_input: TokenStream) -> TokenStream {
    let input = std::fs::read_to_string("../../exp").expect("input file does not exist");
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(|c| match c {
        'L' => quote!{Direction::Left},
        'R' => quote!{Direction::Right},
        _ => panic!("unknown direction '{c}'"),
    }).collect::<Vec<_>>();

    lines.next();
    let mut nodes = Vec::<Ident>::new();
    let mut left = Vec::<Ident>::new();
    let mut right = Vec::<Ident>::new();
    for ln in lines {
        let caps = RE_NODE_LINE.captures(ln).expect("Invalid line for node");
        nodes.push(to_name(caps.name("name").unwrap().as_str().to_string()));
        left.push(to_name(caps.name("left").unwrap().as_str().to_string()));
        right.push(to_name(caps.name("right").unwrap().as_str().to_string()));
    }

    quote!{
        enum Direction {
            Left,
            Right,
        }

        enum Nodes {
            #(#nodes,)*
        }

        impl Nodes {
            fn next(&self, direction: Direction) -> Self {
                match (self, direction) {
                    #( (#nodes, Direction::Left) => Self::#left, )*
                    #( (#nodes, Direction::Right) => Self::#right, )*
                }
            }
        }

        const DIRECTIONS = [#(#directions),*];
    }.into()
}

fn to_name(node: String) -> Ident {
    format_ident!("N_{}", node)
}
