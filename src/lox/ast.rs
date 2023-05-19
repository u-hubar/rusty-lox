use std::marker::PhantomData;

use super::token::{Token, TokenLiteral};

macro_rules! define_ast {
    (
        $(
            $struct_name:ident $visitor_fn_name:ident {
                $($field_name:ident: $field_type:ty),*
            }
        )*
    ) => {
        pub trait Visitor<T> {
            $(fn $visitor_fn_name(&mut self, expr: &$struct_name<T>) -> T;)*
        }

        pub trait Expr<T> {
            fn accept(&self, visitor: &mut dyn Visitor<T>) -> T;
        }

        $(
            pub struct $struct_name<T> {
                $(pub $field_name: $field_type,)*
            }

            impl<T> Expr<T> for $struct_name<T> {
                fn accept(&self, visitor: &mut dyn Visitor<T>) -> T {
                    visitor.$visitor_fn_name(self)
                }
            }
        )*
    };
}

define_ast! {
    BinaryExpr visit_binary_expr {left: Box<dyn Expr<T>>, operator: Token, right: Box<dyn Expr<T>>}
    GroupingExpr visit_grouping_expr {expression: Box<dyn Expr<T>>}
    LiteralExpr visit_literal_expr {value: Option<TokenLiteral>, _phantom: PhantomData<T>}
    UnaryExpr visit_unary_expr {operator: Token, right: Box<dyn Expr<T>>}
}

pub struct AstPrinter {}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr<String>) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            expr.left.accept(self),
            expr.right.accept(self),
        )
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr<String>) -> String {
        format!(
            "({} {})",
            "group",
            expr.expression.accept(self),
        )
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr<String>) -> String {
        match &expr.value {
            Some(v) => v.to_string(),
            None => "nil".to_string(),
        }
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr<String>) -> String {
        format!(
            "({} {})",
            expr.operator.lexeme,
            expr.right.accept(self),
        )
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &dyn Expr<String>) -> String {
        expr.accept(self)
    }
}
