use crate::lexer::tokens::{Literal, Token};


// Just to include the new functions for the rest
#[derive(Debug)]
#[allow(dead_code)]
pub struct Expr {
    binary: Box<Option<Binary>>,
    grouping: Box<Option<Grouping>>,
    unary: Box<Option<Unary>>,
    literal: Option<ExprLiteral>
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Grouping {
    expression: Expr
}

// I already created the Literal struct. Just reuse that.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Unary {
    operator: Token,
    right: Expr
}


impl Expr {
    pub fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        let binary = Box::new(Some(Binary {
            left,
            operator,
            right
        }));
        
        Expr {
            binary,
            grouping: Box::new(None),
            unary: Box::new(None),
            literal: None
        }
    }

    pub fn grouping(expression: Expr) -> Expr {
        let grouping = Box::new(Some(Grouping {
            expression
        }));

        Expr {
            binary: Box::new(None),
            grouping,
            unary: Box::new(None),
            literal: None
        }
    }

    pub fn unary(operator: Token, right: Expr) -> Expr {
        let unary = Box::new(Some(Unary {
            operator,
            right
        }));

        Expr {
            binary: Box::new(None),
            grouping: Box::new(None),
            unary,
            literal: None
        }
    }

    pub fn literal(string_data: Option<Literal>, number_data: Option<f32>, bool_data: Option<bool>) -> Expr {
        
        Expr {
            binary: Box::new(None),
            grouping: Box::new(None),
            unary: Box::new(None),
            literal: Some(ExprLiteral {
                string_data,
                number_data,
                bool_data
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExprLiteral {
    pub string_data: Option<Literal>,
    pub number_data: Option<f32>,
    pub bool_data: Option<bool>
}