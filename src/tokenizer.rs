pub enum Tokens {
  LessThan,      // <
  ClosingTag(String), // </
  GreaterThan,   // >
  SelfClosingTagEnd, // />
  Identifier(String),    // a tag name like `div` or `body`
  Attribute,     // an attribute name like `class` or `href`
  Equals,        // =
  String,        // a string value inside quotes, e.g., "container"
  Text,          // regular text content
  EOF,           // End of File marker
}

// loop through string one character at a time and tokenize it based on what we see

pub fn tokenize(input: &str) -> VecDeque<Tokens> {
    let mut tokens = VecDeque::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            // opening tag or closing tag
            '<' => {
                if chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    tokens.push(Tokens::ClosingTag);
                } else {
                    tokens.push(Tokens::LessThan);
                }
            },
            '>' => tokens.push(Tokens::GreaterThan),
            '=' => tokens.push(Tokens::Equals),
            '"' => {
                let mut string_value = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char == '"' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    string_value.push(chars.next().unwrap());
                }
                tokens.push(Tokens::String);
            },
            _ if c.is_alphanumeric() || c == '_' => {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Tokens::Identifier);
            },
            // skip whitespace
            _ if c.is_whitespace() => continue, 
            // self closing tag
            '/' if chars.peek() == Some(&'>') => {
                chars.next(); // consume '>'
                tokens.push(Tokens::SelfClosingTagEnd);
            },
            // need to store all text
            _ => 
            {
              // store identifier
              if (tokens..back() == Some(Token::LessThan) || tokens.back() == Some(Token::ClosingTag)) 
              {
                let mut identifier = String::new();
                identifier.push(c);
                while let Some(&next_char) = chars.peek() {
                    if c.is_whitespace() 
                    {
                      continue;
                    }
                    if next_char.is_alphanumeric() || next_char.peak() != '>' {
                        identifier.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Tokens::Identifier(identifier));
              }
            
            tokens.push(Tokens::Text), // treat any other character as text
        }
    }
    
    tokens.push(Tokens::EOF); // End of File marker
    tokens
}