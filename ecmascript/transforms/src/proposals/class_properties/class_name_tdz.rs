use crate::{helpers::Helpers, util::ExprFactory};
use ast::*;
use swc_common::{Fold, FoldWith, DUMMY_SP};

pub(super) struct ClassNameTdzFolder<'a> {
    pub helpers: &'a Helpers,
    pub class_name: &'a Ident,
}

impl<'a> Fold<Expr> for ClassNameTdzFolder<'a> {
    fn fold(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Ident(i) => {
                //

                if i.sym == self.class_name.sym {
                    self.helpers.class_name_tdz_error();

                    return Expr::Seq(SeqExpr {
                        span: DUMMY_SP,
                        exprs: vec![
                            box Expr::Call(CallExpr {
                                span: DUMMY_SP,
                                callee: quote_ident!("_classNameTDZError").as_callee(),
                                args: vec![Lit::Str(Str {
                                    span: i.span,
                                    value: i.sym.clone(),
                                    has_escape: false,
                                })
                                .as_arg()],

                                type_args: Default::default(),
                            }),
                            box Expr::Ident(i),
                        ],
                    });
                } else {
                    Expr::Ident(i)
                }
            }

            _ => expr.fold_children(self),
        }
    }
}
