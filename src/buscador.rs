use std::collections::{HashMap};
use crate::tokenizer::Tokens;
use crate::praser::Node;

// vamos a buscar en el árbol por una hoja specifica. 

fn find_node<'a,F>(node: &'a Node, predicate: &F) -> Option<&'a Node>
where
    F: Fn(&Node) -> bool, // this is telling us what the template has to be, Fn -> Is a rust trait 
{
    // Check if the current node matches the predicate
    if predicate(node) {
        return Some(node);
    }

    // If it's an Element node, recursively search its children
    if let Node::Element { children, .. } = node {
        for child in children {
            if let Some(found_node) = find_node(child, predicate) {
                return Some(found_node);
            }
        }
    }

    // No match found in this subtree
    None
}

pub fn find_by_tag_name<'a> (tag_name: &'a str, root_node: &'a Node) -> Option<&'a Node> {
    find_node(&root_node, &|node| {
        if let Node::Element { tag_name: node_tag_name, .. } = node {
            node_tag_name == tag_name
        } else {
            false
        }
    })
}

pub fn find_by_attribute <'a>(attribute_name: &'a str, attribute_value: &'a str, root_node: &'a Node) -> Option<&'a Node> {
    find_node(&root_node, &|node| {
        if let Node::Element { attributes, .. } = node {
            attributes.get(attribute_name) == Some(&attribute_value.to_string())
        } else {
            false
        }
    })
}

pub fn find_by_text <'a> (text: &'a str, root_node: &'a Node) -> Option<&'a Node> {
    find_node(&root_node, &|node| {
        if let Node::Text(node_text)= node {
            node_text == text
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests{

    use crate::praser::*;
    use crate::buscador::*;
    #[test]
    fn test_find_by_text_found() {
        let AST:Vec<Node> = vec![
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

    // We know the first element of the AST is the root 'html' node.
        let root_node = &AST[0];
        let text_to_find = "Tokenizer Test Page";
        let found_node = find_by_text(text_to_find, root_node);

        // Check that a node was found.
        assert!(found_node.is_some());

        // Get the found node and verify its content.
        let node = found_node.unwrap();
        if let Node::Text(found_text) = node {
            assert_eq!(found_text, text_to_find);
        } else {
            // This case should not be reached if the function works correctly.
            panic!("Found node is not a text node!");
        }
    }
}   