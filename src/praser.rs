use std::collections::{HashMap, VecDeque};
use crate::tokenizer::Tokens;

// A generic name-value pair for attributes.
pub type Attributes = HashMap<String, String>;

// The core Node enum. This is the heart of your AST.
#[derive(Debug)]
pub enum Node {
    // Represents an HTML element (e.g., <p>, <h1>).
    Element {
        tag_name: String,
        attributes: Attributes,
        children: Vec<Node>,
    },
    // Represents the text content within an element.
    Text(String),
}

// go through each token and make nodes

pub fn graph_creator(tokens:&mut VecDeque<Tokens>) -> Node {
    // Placeholder implementation.
    let mut tag_name = String::new();
    let mut attributes: Attributes = HashMap::new();
    let mut children: Vec<Node> = Vec::new();

    // loop through tokens.
    while !tokens.is_empty() {
      // grab the first token, which should be a '<'
      let token = tokens.pop_front();
      print!("parsing token {:?}\n", token);
      match token {
        Some (Tokens::ClosingTag) => {
          print!("closing tag found\n");
          // closing tag
          if let Some(Tokens::Identifier(name)) = tokens.pop_front() {
            // ensure the closing tag matches the opening tag
              // consume greater than
            tokens.pop_front(); // remove '>'
            
            return Node::Element {
              tag_name,
              attributes,
              children,};
          }
          else {
            panic!("Expected tag name after closing tag");
          }
        },
        Some(Tokens::GreaterThan) => {
          // end of opening tag, continue to parse children or text
          print!("End of opening tag or closing tag found\n");
          continue;
        },
        Some (Tokens::LessThan) => {
            // opening tag, is it a new opening tag?
            if tag_name == "" {
              // grab the tag name
              print!("New node found and we dont have a node yet\n");
              if let Some(Tokens::Identifier(name)) = tokens.pop_front() {
                tag_name = name;
              }
            }
            else {
              // it's a new opening tag, so we need to create a new node
              print!("New node found so we're going to recurse\n");
              // add back less than so that it can be used in the recursive call
              tokens.push_front(Tokens::LessThan);
              children.push(graph_creator(tokens));
              continue; // continue to the next iteration
            }
        },
        Some(Tokens::Attribute(attribute_name)) => {
          print!("attribute found: {}\n", attribute_name);
          // handle attributes
          // e.g., class="my-class"
          tokens.pop_front(); // remove '='
          if let Some(Tokens::String(value)) = tokens.pop_front() {
            print!("attribute value found: {}\n", value);
            attributes.insert(attribute_name, value);
          }
          continue; 
        },
        Some(Tokens::SelfClosingTagEnd)=> {
          // self-closing tag, return the node
          print!("self closing tag found\n");
          return Node::Element {
            tag_name,
            attributes,
            children,
          };
        },
        Some(Tokens::EOF) => {
          print!("got to end of File \n");
          // end of file
          break;
        },
        Some(Tokens::Text(text)) => {
          
          // text content inside an element
          print!("text found: {}\n", text);
          children.push(Node::Text(text));
          continue;
        },
        _ => {
          print!("Unhandled token or end of tokens\n {:?}\n", token);
          break;
        
        } // end of tokens
      }
  }
  return Node::Element { tag_name, attributes, children };
  // If we reach here, it means we didn't find a closing tag
}

#[cfg(test)]
mod tests {
  use std::collections::{HashMap, VecDeque};
  use crate::praser::*;
  use crate::tokenizer::Tokens;
  #[test]
  fn test_parser()
  {
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
      let ast = vec![
        Node::Element {
            tag_name: "html".to_string(),
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert("lang".to_string(), "en".to_string());
                attrs
            },
            children: vec![
                Node::Element {
                    tag_name: "head".to_string(),
                    attributes: HashMap::new(),
                    children: vec![
                        Node::Element {
                            tag_name: "meta".to_string(),
                            attributes: {
                                let mut attrs = HashMap::new();
                                attrs.insert("charset".to_string(), "UTF-8".to_string());
                                attrs
                            },
                            children: vec![],
                        },
                        Node::Element {
                            tag_name: "meta".to_string(),
                            attributes: {
                                let mut attrs = HashMap::new();
                                attrs.insert("name".to_string(), "viewport".to_string());
                                attrs.insert("content".to_string(), "width=device-width, initial-scale=1.0".to_string());
                                attrs
                            },
                            children: vec![],
                        },
                        Node::Element {
                            tag_name: "title".to_string(),
                            attributes: HashMap::new(),
                            children: vec![
                                Node::Text("Tokenizer Test Page".to_string()),
                            ],
                        },
                        Node::Element {
                            tag_name: "style".to_string(),
                            attributes: HashMap::new(),
                            children: vec![
                                Node::Text(r#"body { font-family: sans-serif; margin: 20px; background-color: #f4f4f4; color: #333; } .container { max-width: 800px; margin: 0 auto; background-color: #fff; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1); } h1 { color: #0056b3; } p { line-height: 1.6; } code { background-color: #e0e0e0; padding: 2px 4px; border-radius: 3px; }"#.to_string()),
                            ],
                        },
                    ],
                },
                Node::Element {
                    tag_name: "body".to_string(),
                    attributes: HashMap::new(),
                    children: vec![
                        Node::Element {
                            tag_name: "div".to_string(),
                            attributes: {
                                let mut attrs = HashMap::new();
                                attrs.insert("class".to_string(), "container".to_string());
                                attrs
                            },
                            children: vec![
                                Node::Element {
                                    tag_name: "h1".to_string(),
                                    attributes: HashMap::new(),
                                    children: vec![
                                        Node::Text("This is some sample text".to_string()),
                                    ],
                                },
                                Node::Element {
                                    tag_name: "p".to_string(),
                                    attributes: {
                                        let mut attrs = HashMap::new();
                                        attrs.insert("class".to_string(), "intro".to_string());
                                        attrs
                                    },
                                    children: vec![
                                        Node::Text("Welcome to this ".to_string()),
                                        Node::Element {
                                            tag_name: "span".to_string(),
                                            attributes: {
                                                let mut attrs = HashMap::new();
                                                attrs.insert("id".to_string(), "test_span".to_string());
                                                attrs
                                            },
                                            children: vec![
                                                Node::Text("simple page".to_string()),
                                            ],
                                        },
                                        Node::Text(" for tokenizer testing.".to_string()),
                                    ],
                                },
                                Node::Element {
                                    tag_name: "p".to_string(),
                                    attributes: HashMap::new(),
                                    children: vec![
                                        Node::Text("Here's a paragraph with a ".to_string()),
                                        Node::Element {
                                            tag_name: "a".to_string(),
                                            attributes: {
                                                let mut attrs = HashMap::new();
                                                attrs.insert("target".to_string(), "_blank".to_string());
                                                attrs.insert("href".to_string(), "https://example.com".to_string());
                                                attrs.insert("data-info".to_string(), "link".to_string());
                                                attrs
                                            },
                                            children: vec![
                                                Node::Text("link".to_string()),
                                            ],
                                        },
                                        Node::Text(".".to_string()),
                                    ],
                                },
                                Node::Element {
                                    tag_name: "img".to_string(),
                                    attributes: {
                                        let mut attrs = HashMap::new();
                                        attrs.insert("src".to_string(), "https://placehold.co/150x50/cccccc/333333?text=Image".to_string());
                                        attrs.insert("alt".to_string(), "Placeholder Image".to_string());
                                        attrs
                                    },
                                    children: vec![],
                                },
                                Node::Element {
                                    tag_name: "br".to_string(),
                                    attributes: HashMap::new(),
                                    children: vec![],
                                },
                                Node::Element {
                                    tag_name: "div".to_string(),
                                    attributes: {
                                        let mut attrs = HashMap::new();
                                        attrs.insert("id".to_string(), "dynamicContent".to_string());
                                        attrs.insert("style".to_string(), "background-color: lightblue;".to_string());
                                        attrs
                                    },
                                    children: vec![
                                        Node::Element {
                                            tag_name: "p".to_string(),
                                            attributes: HashMap::new(),
                                            children: vec![
                                                Node::Text("Another paragraph inside a div.".to_string()),
                                            ],
                                        },
                                    ],
                                },
                                Node::Element {
                                    tag_name: "p".to_string(),
                                    attributes: HashMap::new(),
                                    children: vec![
                                        Node::Text("This is the final content.".to_string()),
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ];
    let mut test_tokens_deque: VecDeque<Tokens> = test_tokens.into_iter().collect();
    let output_graph = graph_creator(&mut test_tokens_deque);

    // assert the output graph is the same as the ast
      print!("Output Graph: {:#?}\n", output_graph);
      fn compare_nodes(expected: &Node, actual: &Node, path: &str) {
      match (expected, actual) {
        (Node::Element { tag_name: e_tag, attributes: e_attrs, children: e_children },
         Node::Element { tag_name: a_tag, attributes: a_attrs, children: a_children }) => {
            
            assert_eq!(e_tag, a_tag, "Tag name mismatch at {}", path);
            assert_eq!(e_attrs, a_attrs, "Attributes mismatch for tag {} at {}", e_tag, path);
            
            for (i, (e_child, a_child)) in e_children.iter().zip(a_children.iter()).enumerate() {
                compare_nodes(e_child, a_child, &format!("{}/{}[{}]", path, e_tag, i));
            }
            
            assert_eq!(e_children.len(), a_children.len(), 
                "Different number of children for tag {} at {}", e_tag, path);
        },
        (Node::Text(e_text), Node::Text(a_text)) => {
            assert_eq!(e_text, a_text, "Text content mismatch at {}", path);
        },
        _ => {
            panic!("Node type mismatch at {}", path);
        }
    }
}

compare_nodes(&ast[0], &output_graph, "");  }
}