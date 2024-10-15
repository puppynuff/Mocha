use std::collections::HashMap;

use super::tokens::{ Token, TokenType, Literal };

// Rust object type thing is pretty wierd...
// Thats what I get for using a object in a function based thing
// Figuring out how to instantiate this without all of them being public was a paaaain.
pub struct Scanner {
    pub source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>
}

impl Scanner {
    // Ehhh
    // This just tells other functions to do it
    // Like a chain of command, you go through 5 different functions that just tell another to do it for them
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        let eof_token : Token = Token {
            token_type: TokenType::EOF, 
            lexeme: "".to_string(),
            literal: None,
            line: self.line
        };

        self.tokens.push(eof_token);

        let tokens = self.tokens.clone(); // This fucker here did it (tokens.rs, line 30)

        // Sometimes I forget you can return stuff like this in rust.
        tokens
    }

    // Such a stupidly complicated function, isn't it?
    // Like this took me hours to finally understand /j
    fn is_at_end(&mut self) -> bool {
        return self.current >= self.source.len().try_into().unwrap()
    }

    fn scan_token(&mut self) {
        let character : char = self.advance();

        // The match of doom.
        match character {
            // Single character, no peeking
            '(' => { self.add_token_by_type(TokenType::LEFT_PAREN) },
            ')' => { self.add_token_by_type(TokenType::RIGHT_PAREN) },
            '{' => { self.add_token_by_type(TokenType::LEFT_BRACE) },
            '}' => { self.add_token_by_type(TokenType::RIGHT_BRACE) },
            '[' => { self.add_token_by_type(TokenType::LEFT_BRACKET) },
            ']' => { self.add_token_by_type(TokenType::RIGHT_BRACKET) },
            ',' => { self.add_token_by_type(TokenType::COMMA) },
            '.' => { self.add_token_by_type(TokenType::DOT) },
            '-' => { self.add_token_by_type(TokenType::MINUS) },
            '+' => { self.add_token_by_type(TokenType::PLUS) },
            ';' => { self.add_token_by_type(TokenType::SEMICOLON) },
            '*' => { self.add_token_by_type(TokenType::STAR) },

            // Match next
            '!' => {
                if self.match_next('=') {
                    self.add_token_by_type(TokenType::BANG_EQUAL);
                    return;
                }

                self.add_token_by_type(TokenType::BANG);
                return;
            }

            '=' => {
                if self.match_next('=') {
                    self.add_token_by_type(TokenType::EQUAL_EQUAL);
                    return;
                }

                self.add_token_by_type(TokenType::EQUAL);
                return;
            }

            '<' => {
                if self.match_next('=') {
                    self.add_token_by_type(TokenType::LESS_EQUAL);
                    return;
                }

                self.add_token_by_type(TokenType::LESS);
                return;
            }

            '>' => {
                if self.match_next('=') {
                    self.add_token_by_type(TokenType::GREATER_EQUAL);
                    return;
                }

                self.add_token_by_type(TokenType::GREATER);
                return;
            }

            // Full line matching (Mostly just comments.)
            '/' => {
                if self.match_next ('/') {
                    while self.peek(0) != '\n' {
                        self.advance();
                    }
                    return;
                }


                if self.match_next('*') {
                    while self.peek(0) != '*' && self.peek(1) != '/' {
                        self.advance();
                    }

                    self.advance();
                    self.advance();
                    
                    return;
                }

                self.add_token_by_type(TokenType::SLASH);
            }

            '"' => {
                self.handle_string();

                return;
            }

            // Ignore whitespace
            ' ' => {
                return;
            } 

            '\r' => {
                return;
            }

            '\t' => {
                return;
            }

            '\n' => {
                self.line += 1;
                return;
            }

            _ => { 
                if is_digit(character) {
                    self.number();
                } else if is_alphabetical(character) {
                    self.identifier();
                } else {
                    self.mochastl_error(format!("Unexpected character {}", character));
                }

                return;
            }
        }
    }

    // Returns the next characer in source.
    fn advance(&mut self) -> char {
        let char =  self.source.chars().nth(self.current.try_into().unwrap()).unwrap();
        self.current += 1;
        return char;
    }

    // This is just so you dont have to add the literal every time.
    fn add_token_by_type(&mut self, token_type: TokenType) {
        self.add_token(token_type, None);
    }

    // Figuring out what I needed for the literal object was a pain
    // But now its no longer a pain
    fn add_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text: String = self.source.as_str()[self.start..self.current].to_string();

        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line
        })
    }

    // Just checks to see if the next char is the same as you ask for.
    fn match_next(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current.try_into().unwrap()).unwrap() != expected_char {
            return false;
        }

        self.current += 1;
        return true;

    }

    // WHAT IS THE NEXT CHAR????
    fn peek(&mut self, skip: usize) -> char {
        if self.is_at_end() { return '\0' };

        return self.source.chars().nth(self.current + skip).unwrap();
    }

    // This one function led me down the Option<> rabbit hole
    // It took me an hour
    fn handle_string(&mut self) {
        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' { self.line += 1; };

            self.advance();
        }

        if self.is_at_end() {
            self.mochastl_error("Unterminated string".to_string());
            return;
        }

        self.advance();


        let value: String = self.source.as_str()[self.start + 1..self.current - 1].to_string();

        self.add_token(TokenType::STRING, Some(Literal {
            string_data: Some(value),
            number_data: None,
            bool_data: None
        }));
    }

    // Ehh didn't need this.
    // Just felt like it for some reason.
    pub fn mochastl_error(&mut self, error_string: String) {
        println!("{}, at line {}, character number {}", error_string, self.line, self.current);

        return;
    }

    // When strings arent enough, you can use a number
    fn number(&mut self) {
        while is_digit(self.peek(0)) {
            self.advance();
        }

        if self.peek(0) == '.' && is_digit(self.peek(1)) {
            self.advance();

            while is_digit(self.peek(0)) {
                self.advance();
            }
        }

        let value: f32 = self.source.as_str()[self.start..self.current].to_string().parse().unwrap();

        self.add_token(TokenType::NUMBER, Some(Literal {
            string_data: None,
            number_data: Some(value),
            bool_data: None
        }));
    }

    // Reserved words my beloved.
    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek(0)) { 
            self.advance();
        }

        let text : String = self.source.as_str()[self.start..self.current].to_string();

        let option_token_type: Option<&TokenType>  = self.keywords.get(&text);

        let mut token_type: TokenType = TokenType::IDENTIFIER;

        if option_token_type.is_some() {
            token_type = option_token_type.unwrap().to_owned();
        }

        self.add_token_by_type(token_type);
    }

    // This is how you instantiate private variables in structs btw.
    pub fn new(source: String) -> Scanner {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        
        keywords.insert("and".to_string(), TokenType::AND);
        keywords.insert("class".to_string(), TokenType::CLASS);
        keywords.insert("else".to_string(), TokenType::ELSE);
        keywords.insert("false".to_string(), TokenType::FALSE);
        keywords.insert("for".to_string(), TokenType::FOR);
        keywords.insert("function".to_string(), TokenType::FUNCTION);
        keywords.insert("if".to_string(), TokenType::IF);
        keywords.insert("null".to_string(), TokenType::NULL);
        keywords.insert("or".to_string(), TokenType::OR);
        keywords.insert("print".to_string(), TokenType::PRINT); // Change this to a standard library function later.
        keywords.insert("return".to_string(), TokenType::RETURN);
        keywords.insert("super".to_string(), TokenType::SUPER);
        keywords.insert("this".to_string(), TokenType::THIS);
        keywords.insert("true".to_string(), TokenType::TRUE);
        keywords.insert("let".to_string(), TokenType::LET);
        keywords.insert("while".to_string(), TokenType::WHILE);

        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords
        }
    }
}


// I was stupid and put these three in the struct
// And then realized I was stupid for doing that and took it out
// Saved like 50 lines of code from work arounds :]
fn is_digit(character: char) -> bool {
    return character >= '0' && character <= '9'
}

fn is_alphabetical(character: char) -> bool {
    return (character >= 'a' && character <= 'z') ||
           (character >= 'A' && character <= 'Z') ||
           character == '_';
}

fn is_alpha_numeric(character: char) -> bool {
    return is_alphabetical(character) || is_digit(character);
}