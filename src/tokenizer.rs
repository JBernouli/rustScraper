use std::collections::VecDeque;


#[derive(Debug)]
pub enum Tokens {
  LessThan,      // <
  ClosingTag, // </
  GreaterThan,   // >
  SelfClosingTagEnd, // />
  Identifier(String),    // a tag name like `div` or `body`
  Attribute(String),     // an attribute name like `class` or `href`
  Equals,        // =
  String(String),        // a string value inside quotes, e.g., "container"
  Text(String),          // regular text content
  EOF,           // End of File marker
}



// loop through string one character at a time and tokenize it based on what we see

pub fn tokenize(input: &str) -> VecDeque<Tokens> {
    let mut needs_self_closing = false;
    let mut tokens:VecDeque<Tokens> = VecDeque::new();
    let mut chars = input.chars().peekable();
    const VOID_ELEMENTS:[&str;14]  = ["area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source", "track", "wbr"];
    // println!("Entering main loop");
    while let Some(c) = chars.next() {
        match c {
            // opening tag or closing tag
            '<' => {
                if chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    tokens.push_back(Tokens::ClosingTag);
                } 
                else if chars.peek() == Some(&'!') {
                  // Handle comments or DOCTYPE
                  chars.next(); // consume '!'
                  if chars.peek() == Some(&'-') {
                      chars.next(); // consume '-'
                      if chars.peek() == Some(&'-') {
                          chars.next(); // consume second '-'
                          // println!("Entering loop for comment");
                          while let Some(next_char) = chars.next() {
                              if next_char == '-' && chars.peek() == Some(&'-') {
                                  chars.next(); // consume second '-'
                                  if chars.peek() == Some(&'>') {
                                      chars.next(); // consume '>'
                                      break;
                                  }
                              }
                          }
                          continue; // skip adding any token for comments
                      }
                  }
                  // println!("Entering loop for comment or DOCTYPE");
                  while let Some(next_char) = chars.next() {
                      if next_char == '>' {
                          break;
                      }
                  } 
                }else {
                    tokens.push_back(Tokens::LessThan);
                }
            },
            '>' => {
              if needs_self_closing {
                needs_self_closing = false;
                tokens.push_back(Tokens::SelfClosingTagEnd);
                continue; // skip adding GreaterThan token for self-closing tags
              }  
              else
              {
                tokens.push_back(Tokens::GreaterThan)
              }
            
            },
            '=' => tokens.push_back(Tokens::Equals),
            '"' => {
                // println!("Entering loop in quotes");
                let mut string_value = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char == '"' {
                        chars.next(); // consume closing quote
                        break;
                    }
                    string_value.push(chars.next().unwrap());
                }
                tokens.push_back(Tokens::String(string_value));
            },
            // self closing tag
            '/' if chars.peek() == Some(&'>') => {
                chars.next(); // consume '>'
                tokens.push_back(Tokens::SelfClosingTagEnd);
                needs_self_closing = false; // reset the flag
            },
            // need to store all text
            _ => {
              // store identifier
              if c.is_whitespace() {
                  continue; // skip whitespace
              }
              let mut identifier = String::new();
              
              identifier.push(c);
              while let Some(&next_char) = chars.peek() 
              {
                  if (next_char.is_whitespace() &&  matches!(tokens.back(), Some(Tokens::LessThan))) || next_char == '/' || next_char == '>' || next_char == '<' || next_char == '=' || next_char == '"' 
                  {
                      break;
                  }
                  identifier.push(chars.next().unwrap());
              }
              if let Some(next_char) = chars.peek()
              {
                match next_char
                {
                  c if c.is_whitespace() && matches!(tokens.back(),Some(Tokens::Identifier(_))) => tokens.push_back(Tokens::Attribute(identifier)),
                  '=' => tokens.push_back(Tokens::Attribute(identifier)),
                  x if matches!(tokens.back(), Some(Tokens::LessThan)) || matches!(tokens.back(), Some(Tokens::ClosingTag)) => 
                  {
                    if VOID_ELEMENTS.contains(&identifier.as_str()) {
                      needs_self_closing = true;
                      println!("Detected void element: {}", identifier);
                    }
                    tokens.push_back(Tokens::Identifier(identifier));
                  },
                  _ => {
                    tokens.push_back(Tokens::Text(identifier))},
                }
              }
            }
        }
    }
    
    tokens.push_back(Tokens::EOF); // End of File marker
    tokens    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
      const HTML_CONTENT: &str = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0"><title>Tokenizer Test Page</title><style>body {font-family: sans-serif;margin: 20px;background-color: #f4f4f4;color: #333;}.container {max-width: 800px;margin: 0 auto;background-color: #fff;padding: 20px;border-radius: 8px;box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);}h1 {color: #0056b3;}p {line-height: 1.6;}code {background-color: #e0e0e0;padding: 2px 4px;border-radius: 3px;}</style></head><body><div class="container"><!-- LessThan, GreaterThan, Identifier (html, head, body, div, p, a, img) --><!-- ClosingTag (</div>, </body>, </html>) --><h1>This is some sample text</h1><p class="intro">Welcome to this <span id="test_span">simple page</span> for tokenizer testing.</p><p>Here's a paragraph with a <a href="https://example.com" target="_blank" data-info="link">link</a>.</p><!-- SelfClosingTagEnd (/>) --><img src="https://placehold.co/150x50/cccccc/333333?text=Image" alt="Placeholder Image" /><br/><!-- Attribute (class, id, href, target, data-info, src, alt) --><!-- Equals (=) --><!-- String ("intro", "test_span", "https://example.com", "_blank", "link", "https://placehold.co/150x50/cccccc/333333?text=Image", "Placeholder Image") --><!-- Text (This is some sample text, Welcome to this, simple page, for tokenizer testing., Here's a paragraph with a, link, .) --><!-- A more complex div --><div id="dynamicContent" style="background-color: lightblue;"><p>Another paragraph inside a div.</p></div><p>This is the final content.</p><!-- EOF will be at the very end of the file after </html> --></div></body></html>"#;
        
        let result = tokenize(HTML_CONTENT);
        let test_tokens = [
          Tokens::LessThan,
          Tokens::Identifier("html".to_string()),
          Tokens::Attribute("lang".to_string()),
          Tokens::Equals,
          Tokens::String("en".to_string()),
          Tokens::GreaterThan,
      
          Tokens::LessThan,
          Tokens::Identifier("head".to_string()),
          Tokens::GreaterThan,
      
          Tokens::LessThan,
          Tokens::Identifier("meta".to_string()),
          Tokens::Attribute("charset".to_string()),
          Tokens::Equals,
          Tokens::String("UTF-8".to_string()),
          Tokens::SelfClosingTagEnd, // Corrected from Tokens::GreaterThan
      
          Tokens::LessThan,
          Tokens::Identifier("meta".to_string()),
          Tokens::Attribute("name".to_string()),
          Tokens::Equals,
          Tokens::String("viewport".to_string()),
          Tokens::Attribute("content".to_string()),
          Tokens::Equals,
          Tokens::String("width=device-width, initial-scale=1.0".to_string()),
          Tokens::SelfClosingTagEnd, // Corrected from Tokens::GreaterThan
      
          Tokens::LessThan,
          Tokens::Identifier("title".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("Tokenizer Test Page".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("title".to_string()),
          Tokens::GreaterThan,
      
          Tokens::LessThan,
          Tokens::Identifier("style".to_string()),
          Tokens::GreaterThan,
          // The entire CSS content would be parsed as a single Text token by your enum
          Tokens::Text(r#"body { font-family: sans-serif; margin: 20px; background-color: #f4f4f4; color: #333; } .container { max-width: 800px; margin: 0 auto; background-color: #fff; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); } h1 { color: #0056b3; } p { line-height: 1.6; } code { background-color: #e0e0e0; padding: 2px 4px; border-radius: 3px; }"#.to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("style".to_string()),
          Tokens::GreaterThan,
          Tokens::ClosingTag,
          Tokens::Identifier("head".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("body".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("div".to_string()),
          Tokens::Attribute("class".to_string()),
          Tokens::Equals,
          Tokens::String("container".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("h1".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("This is some sample text".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("h1".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("p".to_string()),
          Tokens::Attribute("class".to_string()),
          Tokens::Equals,
          Tokens::String("intro".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("Welcome to this ".to_string()),
          Tokens::LessThan,
          Tokens::Identifier("span".to_string()),
          Tokens::Attribute("id".to_string()),
          Tokens::Equals,
          Tokens::String("test_span".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("simple page".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("span".to_string()),
          Tokens::GreaterThan,
          Tokens::Text(" for tokenizer testing.".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("Here's a paragraph with a ".to_string()),
          Tokens::LessThan,
          Tokens::Identifier("a".to_string()),
          Tokens::Attribute("href".to_string()),
          Tokens::Equals,
          Tokens::String("https://example.com".to_string()),
          Tokens::Attribute("target".to_string()),
          Tokens::Equals,
          Tokens::String("_blank".to_string()),
          Tokens::Attribute("data-info".to_string()),
          Tokens::Equals,
          Tokens::String("link".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("link".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("a".to_string()),
          Tokens::GreaterThan,
          Tokens::Text(".".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("img".to_string()),
          Tokens::Attribute("src".to_string()),
          Tokens::Equals,
          Tokens::String("https://placehold.co/150x50/cccccc/333333?text=Image".to_string()),
          Tokens::Attribute("alt".to_string()),
          Tokens::Equals,
          Tokens::String("Placeholder Image".to_string()),
          Tokens::SelfClosingTagEnd,
          Tokens::LessThan,
          Tokens::Identifier("br".to_string()),
          Tokens::SelfClosingTagEnd,
          Tokens::LessThan,
          Tokens::Identifier("div".to_string()),
          Tokens::Attribute("id".to_string()),
          Tokens::Equals,
          Tokens::String("dynamicContent".to_string()),
          Tokens::Attribute("style".to_string()),
          Tokens::Equals,
          Tokens::String("background-color: lightblue;".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("Another paragraph inside a div.".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::ClosingTag,
          Tokens::Identifier("div".to_string()),
          Tokens::GreaterThan,
          Tokens::LessThan,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::Text("This is the final content.".to_string()),
          Tokens::ClosingTag,
          Tokens::Identifier("p".to_string()),
          Tokens::GreaterThan,
          Tokens::ClosingTag,
          Tokens::Identifier("div".to_string()),
          Tokens::GreaterThan,
          Tokens::ClosingTag,
          Tokens::Identifier("body".to_string()),
          Tokens::GreaterThan,
          Tokens::ClosingTag,
          Tokens::Identifier("html".to_string()),
          Tokens::GreaterThan,
          Tokens::EOF,
        ];
      
   
    // debug_assert_eq!(result.len(), test_tokens.len(), "result: {:?}", result);
    

    for (i, token) in result.iter().enumerate() {
        match (token, &test_tokens[i]) {
            (Tokens::LessThan, Tokens::LessThan) => (),
            (Tokens::GreaterThan, Tokens::GreaterThan) => (),
            (Tokens::ClosingTag, Tokens::ClosingTag) => (),
            (Tokens::SelfClosingTagEnd, Tokens::SelfClosingTagEnd) => (),
            (Tokens::Equals, Tokens::Equals) => (),
            (Tokens::EOF, Tokens::EOF) => (),
            (Tokens::Identifier(a), Tokens::Identifier(b)) => assert_eq!(a, b),
            (Tokens::Attribute(a), Tokens::Attribute(b)) => assert_eq!(a, b),
            (Tokens::String(a), Tokens::String(b)) => assert_eq!(a, b),
            (Tokens::Text(a), Tokens::Text(b)) => assert_eq!(a.replace(" ",""), b.replace(" ", "")),
            _ => panic!("Token mismatch at index {}: {:?} != {:?} and result is {:?}", i, token, test_tokens[i], result),
        }
    }
  }
}