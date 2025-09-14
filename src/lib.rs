//lib.rs for scraper
pub mod buscador;
pub mod tokenizer;
pub mod parser;

pub use buscador::*;
pub use tokenizer::*;
pub use parser::*;
use std::collections::{HashMap};
pub use reqwest::Error;

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

pub async fn get_html_graph(url: &str) -> (Result<Node, String>) {
    
    let response = reqwest::get(url).await.map_err(|e|e.to_string())?;
    if response.status().is_success() {
        // Read the response body as a string.
        let body = response.text().await.map_err(|e|e.to_string())?;

        let mut tokens = tokenize(&body);
        let graph = graph_creator(&mut tokens);
        Ok(graph)

    } else {
        println!("Request failed with status: {}", response.status());
        return Err("Request failed with status: {}".to_string()  +  &response.status().to_string() );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_html_graph() {
        let url = "https://www.example.com";
        let result = get_html_graph(url).await;
        assert!(result.is_ok());
        let graph = result.unwrap();
        let targets = find_by_tag_name("p", &graph);
        print!("The targets are \n{:?}\n", targets);
        print!("the graph is \n {:?} \n", graph);
        assert_eq!(targets.len(), 2);
        for each_target in targets
        {
            if let Node::Element{tag_name,..} = each_target {
                assert_eq!(tag_name, "p");
            } else {
                // This case should not be reached if the function works correctly.
                panic!("Found node is not correct! {:?}", each_target);
            }
        }
    }
}
