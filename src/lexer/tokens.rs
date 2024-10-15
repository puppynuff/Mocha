// Why cant I just name my types whatever I want to???
// Like forcing me to put it in camel case is aaaaa.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single character types
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    LEFT_BRACKET, RIGHT_BRACKET,


    // One or two character tokens
    BANG, BANG_EQUAL, 
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,

    // Literals
    IDENTIFIER, STRING, NUMBER,

    // Identifiers
    AND, CLASS, ELSE, FALSE, FUNCTION, FOR, IF, NULL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,

    // End of file for interpreter.
    EOF
}


// One line made me have to derive clone to all other structs in this file
#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize
}

// Wow, this function is going to be useless soon!
// Why the fuck do I need to add so much code just to check for a some thing in an option
// Its not too bad, I just feel like it could be more efficient.
impl Token {
    pub fn as_string(&mut self) -> String {
        // Literally just done to make it look prettier
        // Was not needed whatsoever.
        let mut literal_text = "".to_string();

        if self.literal.is_some() {
            let unwrapped_literal = self.literal.as_ref().unwrap();

            if unwrapped_literal.string_data.is_some() {
                literal_text += unwrapped_literal.string_data.clone().unwrap().as_str();
            }

            if unwrapped_literal.number_data.is_some() {
                literal_text += format!("{}", unwrapped_literal.number_data.clone().unwrap()).as_str();
            }
        }
        else {
            literal_text = "None".to_string();
        }
        format!("{:?} {} {}", self.token_type, self.lexeme, literal_text)
    }
}

// These two lines were added after 3 hours of work.
// Just finding what type of data was needed was..... something...
#[derive(Debug, Clone)]
pub struct Literal {
    pub string_data: Option<String>,
    pub number_data: Option<f32>,
    pub bool_data: Option<bool>
}