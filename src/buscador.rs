use crate::tokenizer::Tokens;
use crate::parser::Node;

// vamos a buscar en el árbol por una hoja specifica. 

fn find_node<F>(node: &Node, predicate: &F) -> Option<&Node>
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

fn find_by_tag_name (tag_name: &str, root_node: &Node) -> Option<&Node> {
    find_node(&root_node, &|node| {
        if let Node::Element { tag_name: node_tag_name, .. } = node {
            node_tag_name == tag_name
        } else {
            false
        }
    })
}

fn find_by_attribute (attribute_name: &str, attribute_value: &str, root_node: &Node) -> Option<&Node> {
    find_node(&root_node, &|node| {
        if let Node::Element { attributes, .. } = node {
            attributes.get(attribute_name) == Some(attribute_value)
        } else {
            false
        }
    })
}

fn find_by_text (text: &str, root_node: &Node) -> Option<&Node> {
    find_node(&root_node, &|node| {
        if let Node::Text { text: node_text, .. } = node {
            node_text == text
        } else {
            false
        }
    })
}

