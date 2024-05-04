use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::*;

// assert!(all(a == 0, b))
// should expand to
//
// match (&(a), &(0), &(b)) {
//     (__0, __1, __2) => {
//         use $crate::Expr;
//
//         let __assert_expr = $crate::Finalize {
//             expr: $crate::expr::AndExpr {
//                 lhs: $crate::atomic::EqExpr {
//                     lhs: __0,
//                     rhs: __1,
//                 },
//                 rhs: *__2,
//             },
//             line: (),
//             col: (),
//             file: (),
//         };
//
//         if !__assert_expr.eval_expr() {
//             let __assert_message = $crate::DebugMessage {
//                 result: __assert_expr.result(),
//                 source: $crate::Finalize {
//                     expr: $crate::expr::AndExpr {
//                         lhs: $crate::atomic::EqExpr {
//                             lhs: ::core::stringify!(a),
//                             rhs: ::core::stringify!(0),
//                         },
//                         rhs: ::core::stringify!(b),
//                     },
//                     line: ::core::line!(),
//                     col: ::core::column!(),
//                     file: ::core::file!(),
//                 },
//                 vtable: $crate::vtable_for(&__assert_expr),
//                 debug: $crate::Finalize {
//                     expr: $crate::expr::AndExpr {
//                         lhs: $crate::atomic::EqExpr {
//                             lhs: __0 as *const _ as *const (),
//                             rhs: __1 as *const _ as *const (),
//                         },
//                         rhs: *__2,
//                     },
//                     line: (),
//                     col: (),
//                     file: (),
//                 },
//             };
//             let __marker = $crate::marker(&__assert_message);
//             $crate::panic_failed_assert(
//                 __marker,
//                 __assert_message.result,
//                 __assert_message.source,
//                 __assert_message.vtable,
//                 __assert_message.debug,
//             );
//         }
//     }
// }

struct Operand {
    placeholder_id: Ident,
    expr: Expr,
}

enum AssertExpr {
    BoolExpr(Operand),
    EqExpr(Operand, Operand),
    NeExpr(Operand, Operand),
    LtExpr(Operand, Operand),
    LeExpr(Operand, Operand),
    GtExpr(Operand, Operand),
    GeExpr(Operand, Operand),
    AndExpr(Box<(AssertExpr, AssertExpr)>),
    OrExpr(Box<(AssertExpr, AssertExpr)>),
}

struct Code {
    assert_expr: TokenStream,
    source: TokenStream,
    debug: TokenStream,
}

impl AssertExpr {
    fn code(&self, crate_name: syn::Path) -> Code {
        match self {
            AssertExpr::BoolExpr(Operand {
                placeholder_id,
                expr,
            }) => Code {
                assert_expr: quote! { *#placeholder_id },
                source: quote! { ::core::stringify!(#expr) },
                debug: quote! { *#placeholder_id },
            },
            AssertExpr::EqExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::EqExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::EqExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::EqExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::NeExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::NeExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::NeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::NeExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::LtExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::LtExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::LtExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::LtExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::LeExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::LeExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::LeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::LeExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::GtExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::GtExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::GtExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::GtExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::GeExpr(
                Operand {
                    placeholder_id: left_placeholder_id,
                    expr: left_expr,
                },
                Operand {
                    placeholder_id: right_placeholder_id,
                    expr: right_expr,
                },
            ) => Code {
                assert_expr: quote! {
                    #crate_name::atomic::GeExpr {
                        lhs: (& &#crate_name::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &#crate_name::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    #crate_name::atomic::GeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    #crate_name::atomic::GeExpr {
                        lhs: (#left_placeholder_id) as *const _ as *const (),
                        rhs: (#right_placeholder_id) as *const _ as *const (),
                    }
                },
            },
            AssertExpr::AndExpr(inner) => {
                let (left, right) = &**inner;
                let Code {
                    assert_expr: left_assert_expr,
                    source: left_source,
                    debug: left_debug,
                } = left.code(crate_name.clone());
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    debug: right_debug,
                } = right.code(crate_name.clone());
                Code {
                    assert_expr: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    debug: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_debug),
                            rhs: (#right_debug),
                        }
                    },
                }
            }
            AssertExpr::OrExpr(inner) => {
                let (left, right) = &**inner;
                let Code {
                    assert_expr: left_assert_expr,
                    source: left_source,
                    debug: left_debug,
                } = left.code(crate_name.clone());
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    debug: right_debug,
                } = right.code(crate_name.clone());
                Code {
                    assert_expr: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    debug: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_debug),
                            rhs: (#right_debug),
                        }
                    },
                }
            }
        }
    }
}

fn usize_to_ident(idx: usize) -> Ident {
    Ident::new(&format!("__{idx}"), Span::call_site())
}

fn handle_expr(
    atomics: &mut Vec<Expr>,
    diagnostics: &mut Vec<TokenStream>,
    mut placeholder_id: usize,
    expr: Expr,
) -> (usize, AssertExpr) {
    match expr {
        Expr::Call(expr)
            if match &*expr.func {
                Expr::Path(path) => {
                    path.path
                        .get_ident()
                        .map(|ident| ident.to_string() == "all")
                        == Some(true)
                }
                _ => false,
            } =>
        {
            let mut args = expr.args.into_iter().collect::<Vec<_>>();
            if args.is_empty() {
                let expr = Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(LitBool {
                        value: true,
                        span: Span::call_site(),
                    }),
                });
                atomics.push(expr.clone());
                (
                    placeholder_id + 1,
                    AssertExpr::BoolExpr(Operand {
                        placeholder_id: usize_to_ident(placeholder_id),
                        expr,
                    }),
                )
            } else {
                let mut assert_expr;
                let mut arg_expr;
                (placeholder_id, assert_expr) =
                    handle_expr(atomics, diagnostics, placeholder_id, args.pop().unwrap());
                while let Some(arg) = args.pop() {
                    (placeholder_id, arg_expr) =
                        handle_expr(atomics, diagnostics, placeholder_id, arg);
                    assert_expr = AssertExpr::AndExpr(Box::new((arg_expr, assert_expr)));
                }
                (placeholder_id, assert_expr)
            }
        }
        Expr::Call(expr)
            if match &*expr.func {
                Expr::Path(path) => {
                    path.path
                        .get_ident()
                        .map(|ident| ident.to_string() == "any")
                        == Some(true)
                }
                _ => false,
            } =>
        {
            let mut args = expr.args.into_iter().collect::<Vec<_>>();
            if args.is_empty() {
                let expr = Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(LitBool {
                        value: false,
                        span: Span::call_site(),
                    }),
                });
                atomics.push(expr.clone());
                (
                    placeholder_id + 1,
                    AssertExpr::BoolExpr(Operand {
                        placeholder_id: usize_to_ident(placeholder_id),
                        expr,
                    }),
                )
            } else {
                let mut assert_expr;
                let mut arg_expr;
                (placeholder_id, assert_expr) =
                    handle_expr(atomics, diagnostics, placeholder_id, args.pop().unwrap());
                while let Some(arg) = args.pop() {
                    (placeholder_id, arg_expr) =
                        handle_expr(atomics, diagnostics, placeholder_id, arg);
                    assert_expr = AssertExpr::OrExpr(Box::new((arg_expr, assert_expr)));
                }
                (placeholder_id, assert_expr)
            }
        }
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Eq(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs == #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::EqExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),

        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Ne(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs != #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::NeExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Lt(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs < #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::LtExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Le(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs <= #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::LeExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Gt(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs > #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::GtExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),
        Expr::Binary(ExprBinary {
            left,
            right,
            op: BinOp::Ge(_),
            ..
        }) => (
            {
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(quote! { #lhs >= #rhs });
                atomics.push((*left).clone());
                atomics.push((*right).clone());
                placeholder_id + 2
            },
            AssertExpr::GeExpr(
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id),
                    expr: *left,
                },
                Operand {
                    placeholder_id: usize_to_ident(placeholder_id + 1),
                    expr: *right,
                },
            ),
        ),
        expr => (
            {
                let val = usize_to_ident(placeholder_id);
                diagnostics.push(quote! { *#val });
                atomics.push(expr.clone());
                placeholder_id + 1
            },
            AssertExpr::BoolExpr(Operand {
                placeholder_id: usize_to_ident(placeholder_id),
                expr,
            }),
        ),
    }
}

type FormatArgs = syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>;

struct Args {
    crate_name: syn::Path,
    expr: syn::Expr,
    format_args: Option<FormatArgs>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let crate_name = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let expr = input.parse()?;
        let format_args = if input.is_empty() {
            FormatArgs::new()
        } else {
            input.parse::<syn::token::Comma>()?;
            FormatArgs::parse_terminated(input)?
        };

        let format_args = Some(format_args).filter(|x| !x.is_empty());
        Ok(Self {
            crate_name,
            expr,
            format_args,
        })
    }
}

#[proc_macro]
pub fn assert(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: TokenStream = item.into();
    let Ok(input) = parse2::<Args>(item) else {
        return quote! {
            ::core::compile_error!("invalid arguments");
        }
        .into();
    };

    let crate_name = input.crate_name;
    let args = input
        .format_args
        .map(|punc| punc.into_iter().collect())
        .unwrap_or(Vec::new());
    let body = input.expr;

    let mut atomics = Vec::new();
    let mut diagnostics = Vec::new();
    let assert_expr = handle_expr(&mut atomics, &mut diagnostics, 0, body.clone()).1;
    let atomics = atomics;
    let placeholders = atomics
        .iter()
        .enumerate()
        .map(|(idx, _)| Ident::new(&format!("__{idx}"), Span::call_site()));

    let Code {
        assert_expr,
        source,
        debug,
    } = assert_expr.code(crate_name.clone());

    let outer_block = if args.is_empty() {
        quote! {
            match (#(&(#atomics),)*) {
                (#(#placeholders,)*) => {
                    if false {
                        #(let _: bool = #diagnostics;)*
                    }
                    use #crate_name::Expr;
                    use #crate_name::TryDebugWrap;

                    let __assert_expr = #crate_name::Finalize {
                        expr: #assert_expr,
                        line: (),
                        col: (),
                        file: (),
                    };

                    if !(&&&__assert_expr).eval_expr() {
                        let __assert_message = #crate_name::DebugMessage {
                            result: (&&&__assert_expr).result(),
                            source: &#crate_name::Finalize {
                                expr: #source,
                                line: ::core::line!(),
                                col: ::core::column!(),
                                file: ::core::file!(),
                            },
                            vtable: #crate_name::vtable_for(&__assert_expr),
                            debug: &#crate_name::Finalize {
                                expr: #debug,
                                line: (),
                                col: (),
                                file: (),
                            },
                            message: ::core::format_args!(""),
                        };
                        let __marker = #crate_name::marker(&__assert_message);
                        #crate_name::panic_failed_assert(
                            __marker,
                            __assert_message.result,
                            __assert_message.source,
                            __assert_message.vtable,
                            __assert_message.debug,
                            );
                    }
                }
            }
        }
    } else {
        quote! {
            match (#(&(#atomics),)* ::core::format_args!(#(#args,)*)) {
                (#(#placeholders,)* __message) => {
                    if false {
                        #(let _: bool = #diagnostics;)*
                    }

                    use #crate_name::Expr;
                    use #crate_name::TryDebugWrap;

                    let __assert_expr = #crate_name::Finalize {
                        expr: #assert_expr,
                        line: (),
                        col: (),
                        file: (),
                    };

                    if !(&&&__assert_expr).eval_expr() {
                        let __assert_message = #crate_name::DebugMessage {
                            result: (&&&__assert_expr).result(),
                            source: &#crate_name::Finalize {
                                expr: #source,
                                line: ::core::line!(),
                                col: ::core::column!(),
                                file: ::core::file!(),
                            },
                            vtable: #crate_name::vtable_for(&__assert_expr),
                            debug: &#crate_name::Finalize {
                                expr: #debug,
                                line: (),
                                col: (),
                                file: (),
                            },
                            message: __message,
                        };
                        let __marker = #crate_name::marker(&__assert_message);
                        #crate_name::panic_failed_assert_with_message(
                            __marker,
                            __assert_message.message,
                            __assert_message.result,
                            __assert_message.source,
                            __assert_message.vtable,
                            __assert_message.debug,
                        );
                    }
                }
            }
        }
    };

    outer_block.into()
}
