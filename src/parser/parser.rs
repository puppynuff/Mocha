use crate::lexer::tokens::{Token, TokenType};
use super::expr::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}


impl Parser {
    pub fn parse(&mut self) -> Expr {
        return self.expression();
    }

    // For if I decide to let the parser keep going after an error is found.
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS => {
                    return;
                }
                TokenType::FUNCTION => {
                    return;
                }
                TokenType::LET => {
                    return;
                }
                TokenType::FOR => {
                    return;
                }
                TokenType::IF => {
                    return;
                }

                TokenType::WHILE => {
                    return;
                }

                TokenType::PRINT => {
                    return;
                }

                TokenType::RETURN => {
                    return;
                },

                _ => {
                    self.advance();
                }
            }
        }
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0
        }
    }

    fn expression(&mut self) -> Expr {
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        
        while self.match_types(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator: Token = self.previous();
            let right: Expr = self.comparison();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while self.match_types(vec![TokenType::GREATER, TokenType::GREATER_EQUAL, TokenType::LESS, TokenType::LESS_EQUAL]) {
            let operator: Token = self.previous();
            let right: Expr = self.term();

            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while self.match_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator: Token = self.previous();
            let right = self.factor();
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();

        while self.match_types(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator: Token = self.previous();
            let right = self.unary();
            
            expr = Expr::binary(expr, operator, right);
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator : Token = self.previous();

            let right: Expr = self.unary();

            return Expr::unary(operator, right);
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::FALSE]) {
            return Expr::literal(None, None, Some(false));
        }

        if self.match_types(vec![TokenType::TRUE]) {
            return Expr::literal(None, None, Some(true));
        }

        if self.match_types(vec![TokenType::NULL]) {
            return Expr::literal(None, None, None); // Just check if all options are none to see if it is null.
        }

        if self.match_types(vec![TokenType::NUMBER, TokenType::STRING]) {
            return Expr::literal(self.previous().literal, None, None)
        }

        if self.match_types(vec![TokenType::LEFT_PAREN]) {
            let expr: Expr = self.expression();

            self.consume(TokenType::RIGHT_PAREN, "Expected ')' after expression.".to_string());

            return Expr::grouping(expr);
        }

        panic!("Invalid token {:#?}", self.previous());
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();

                return true;
            }
        }

        return false;
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        return self.previous();
    }

    fn is_at_end(&mut self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&mut self) -> Token {
        return self.tokens[self.current].clone();
    }


    fn previous(&mut self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Token {
        if self.check(token_type) {
            return self.advance();
        }


        throw_parse_error(self.peek(), message);

        panic!();
    }
}

fn throw_parse_error(token: Token, message: String) {
    panic!("Error at line {}, token {}\n{}\n", token.line, token.lexeme, message)
}