extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::Ident;

use regex::Regex;
use once_cell::sync::Lazy;

static RE_NODE_LINE: Lazy::<Regex> = Lazy::new(|| Regex::new(r"^(?<name>[A-Z1-9]+)\s*=\s*\((?<left>[A-Z1-9]+),\s*(?<right>[A-Z1-9]+)\)$").unwrap());


#[proc_macro]
pub fn definitions(_input: TokenStream) -> TokenStream {
    // you need to replace the file name with an absolute path
    let input = std::fs::read_to_string("../../input").expect("input file does not exist");
    let mut lines = input.lines();
    let directions = lines.next().unwrap().chars().map(|c| match c {
        'L' => quote!{Direction::Left},
        'R' => quote!{Direction::Right},
        _ => panic!("unknown direction '{c}'"),
    }).collect::<Vec<_>>();
    let directions_len = directions.len();

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

    let is_start = nodes.iter().map(|n| if n.to_string().ends_with('A') {
        quote!(true)
    } else {
        quote!(false)
    }).collect::<Vec<_>>();
    let is_goal = nodes.iter().map(|n| if n.to_string().ends_with('Z') {
        quote!(true)
    } else {
        quote!(false)
    }).collect::<Vec<_>>();
    let start_nodes = nodes.iter().filter(|n| n.to_string().ends_with('A')).collect::<Vec<_>>();
    let start_nodes_len = start_nodes.len();

    quote!{
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum Direction {
            Left,
            Right,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        enum Node {
            #(#nodes,)*
        }

        impl Node {
            fn next(&self, direction: Direction) -> Self {
                match (self, direction) {
                    #( (Self::#nodes, Direction::Left) => Self::#left, )*
                    #( (Self::#nodes, Direction::Right) => Self::#right, )*
                }
            }

            fn is_start(&self) -> bool {
                match (self) {
                    #( Self::#nodes => #is_start, )*
                }
            }

            fn is_goal(&self) -> bool {
                match (self) {
                    #( Self::#nodes => #is_goal, )*
                }
            }
        }

        const DIRECTIONS: [Direction; (#directions_len)] = [#(#directions),*];
        const START_NODES: [Node; (#start_nodes_len)] = [#(Node::#start_nodes),*];
    }.into()
}

fn to_name(node: String) -> Ident {
    format_ident!("N_{}", node)
}
