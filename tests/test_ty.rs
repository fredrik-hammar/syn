#[macro_use]
mod macros;

use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};
use quote::quote;
use std::iter::FromIterator;
use syn::Type;

#[test]
fn test_mut_self() {
    syn::parse_str::<Type>("fn(mut self)").unwrap();
    syn::parse_str::<Type>("fn(mut self: ())").unwrap();
    syn::parse_str::<Type>("fn(mut self: ...)").unwrap_err();
    syn::parse_str::<Type>("fn(mut self: mut self)").unwrap_err();
    syn::parse_str::<Type>("fn(mut self::T)").unwrap_err();
}

#[test]
fn test_macro_variable_type() {
    // mimics the token stream corresponding to `$ty<T>`
    let tokens = TokenStream::from_iter(vec![
        TokenTree::Group(Group::new(Delimiter::None, quote! { ty })),
        TokenTree::Punct(Punct::new('<', Spacing::Alone)),
        TokenTree::Ident(Ident::new("T", Span::call_site())),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);

    snapshot!(tokens as Type, @r###"
    Type::Path {
        path: Path {
            segments: [
                PathSegment {
                    ident: "ty",
                    arguments: PathArguments::AngleBracketed {
                        args: [
                            Type(Type::Path {
                                path: Path {
                                    segments: [
                                        PathSegment {
                                            ident: "T",
                                            arguments: None,
                                        },
                                    ],
                                },
                            }),
                        ],
                    },
                },
            ],
        },
    }
    "###);
}

#[test]
fn test_group_angle_brackets() {
    // mimics the token stream corresponding to `Option<$ty>`
    let tokens = TokenStream::from_iter(vec![
        TokenTree::Ident(Ident::new("Option", Span::call_site())),
        TokenTree::Punct(Punct::new('<', Spacing::Alone)),
        TokenTree::Group(Group::new(Delimiter::None, quote! { Vec<u8> })),
        TokenTree::Punct(Punct::new('>', Spacing::Alone)),
    ]);

    snapshot!(tokens as Type, @r###"
    Type::Path {
        path: Path {
            segments: [
                PathSegment {
                    ident: "Option",
                    arguments: PathArguments::AngleBracketed {
                        args: [
                            Type(Type::Group {
                                elem: Type::Path {
                                    path: Path {
                                        segments: [
                                            PathSegment {
                                                ident: "Vec",
                                                arguments: PathArguments::AngleBracketed {
                                                    args: [
                                                        Type(Type::Path {
                                                            path: Path {
                                                                segments: [
                                                                    PathSegment {
                                                                        ident: "u8",
                                                                        arguments: None,
                                                                    },
                                                                ],
                                                            },
                                                        }),
                                                    ],
                                                },
                                            },
                                        ],
                                    },
                                },
                            }),
                        ],
                    },
                },
            ],
        },
    }
    "###);
}
