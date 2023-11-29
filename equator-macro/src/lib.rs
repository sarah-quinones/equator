use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::*;

// assert!(all(a == 0, b))
// should expand to
//
// match (&a, &0, &b) {
//     (__0, __1, __2) => {
//         use ::equator::Expr;
//
//         let __assert_expr = ::equator::Finalize {
//             expr: ::equator::expr::AndExpr {
//                 lhs: ::equator::atomic::EqExpr {
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
//             let __assert_message = ::equator::DebugMessage {
//                 result: __assert_expr.result(),
//                 source: ::equator::Finalize {
//                     expr: ::equator::expr::AndExpr {
//                         lhs: ::equator::atomic::EqExpr {
//                             lhs: ::core::stringify!(a),
//                             rhs: ::core::stringify!(0),
//                         },
//                         rhs: ::core::stringify!(b),
//                     },
//                     line: ::core::line!(),
//                     col: ::core::column!(),
//                     file: ::core::file!(),
//                 },
//                 vtable: ::equator::vtable_for(&__assert_expr),
//                 debug: ::equator::Finalize {
//                     expr: ::equator::expr::AndExpr {
//                         lhs: ::equator::atomic::EqExpr {
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
//             let __marker = ::equator::marker(&__assert_message);
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
    fn code(&self) -> Code {
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
                    ::equator::atomic::EqExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::EqExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::EqExpr {
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
                    ::equator::atomic::NeExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::NeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::NeExpr {
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
                    ::equator::atomic::LtExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::LtExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::LtExpr {
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
                    ::equator::atomic::LeExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::LeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::LeExpr {
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
                    ::equator::atomic::GtExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::GtExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::GtExpr {
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
                    ::equator::atomic::GeExpr {
                        lhs: (& &::equator::Wrapper(#left_placeholder_id)).wrap().do_wrap(#left_placeholder_id),
                        rhs: (& &::equator::Wrapper(#right_placeholder_id)).wrap().do_wrap(#right_placeholder_id),
                    }
                },
                source: quote! {
                    ::equator::atomic::GeExpr {
                        lhs: ::core::stringify!(#left_expr),
                        rhs: ::core::stringify!(#right_expr),
                    }
                },
                debug: quote! {
                    ::equator::atomic::GeExpr {
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
                } = left.code();
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    debug: right_debug,
                } = right.code();
                Code {
                    assert_expr: quote! {
                        ::equator::expr::AndExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        ::equator::expr::AndExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    debug: quote! {
                        ::equator::expr::AndExpr {
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
                } = left.code();
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    debug: right_debug,
                } = right.code();
                Code {
                    assert_expr: quote! {
                        ::equator::expr::OrExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        ::equator::expr::OrExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    debug: quote! {
                        ::equator::expr::OrExpr {
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
                    handle_expr(atomics, placeholder_id, args.pop().unwrap());
                while let Some(arg) = args.pop() {
                    (placeholder_id, arg_expr) = handle_expr(atomics, placeholder_id, arg);
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
                    handle_expr(atomics, placeholder_id, args.pop().unwrap());
                while let Some(arg) = args.pop() {
                    (placeholder_id, arg_expr) = handle_expr(atomics, placeholder_id, arg);
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

#[proc_macro]
pub fn assert(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: TokenStream = item.into();
    let Ok(body) = parse2::<Expr>(quote! { __fn_call(#item) }) else {
        return quote! {
            ::core::compile_error!("invalid expression");
        }
        .into();
    };

    let (body, args) = match body {
        Expr::Call(expr) => {
            let mut args = expr.args.into_iter().collect::<Vec<_>>();
            if args.is_empty() {
                return quote! {
                    ::core::compile_error!("invalid expression");
                }
                .into();
            }
            let body = args.remove(0);
            (body, args)
        }
        _ => {
            return quote! {
                ::core::compile_error!("invalid expression");
            }
            .into();
        }
    };

    let mut atomics = Vec::new();
    let assert_expr = handle_expr(&mut atomics, 0, body).1;
    let atomics = atomics;
    let placeholders = atomics
        .iter()
        .enumerate()
        .map(|(idx, _)| Ident::new(&format!("__{idx}"), Span::call_site()));

    let Code {
        assert_expr,
        source,
        debug,
    } = assert_expr.code();

    let outer_block = if args.is_empty() {
        quote! {
            match (#(&#atomics,)*) {
                (#(#placeholders,)*) => {
                    use ::equator::Expr;
                    use ::equator::TryDebugWrap;

                    let __assert_expr = ::equator::Finalize {
                        expr: #assert_expr,
                        line: (),
                        col: (),
                        file: (),
                    };

                    if !__assert_expr.eval_expr() {
                        let __assert_message = ::equator::DebugMessage {
                            result: __assert_expr.result(),
                            source: &::equator::Finalize {
                                expr: #source,
                                line: ::core::line!(),
                                col: ::core::column!(),
                                file: ::core::file!(),
                            },
                            vtable: ::equator::vtable_for(&__assert_expr),
                            debug: &::equator::Finalize {
                                expr: #debug,
                                line: (),
                                col: (),
                                file: (),
                            },
                        };
                        let __marker = ::equator::marker(&__assert_message);
                        ::equator::panic_failed_assert(
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
            match (#(&#atomics,)* ::core::format_args!(#(#args,)*)) {
                (#(#placeholders,)* __message) => {
                    use ::equator::Expr;
                    use ::equator::TryDebugWrap;

                    let __assert_expr = ::equator::Finalize {
                        expr: #assert_expr,
                        line: (),
                        col: (),
                        file: (),
                    };

                    if !__assert_expr.eval_expr() {
                        let __assert_message = ::equator::DebugMessage {
                            result: __assert_expr.result(),
                            source: &::equator::Finalize {
                                expr: #source,
                                line: ::core::line!(),
                                col: ::core::column!(),
                                file: ::core::file!(),
                            },
                            vtable: ::equator::vtable_for(&__assert_expr),
                            debug: &::equator::Finalize {
                                expr: #debug,
                                line: (),
                                col: (),
                                file: (),
                            },
                        };
                        let __marker = ::equator::marker(&__assert_message);
                        ::equator::panic_failed_assert_with_message(
                            __marker,
                            __message,
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
