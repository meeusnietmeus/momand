use kdl::{KdlDocument, KdlNode};
use std::fs;

// PERF: instead of loading whole file in memory, use buffers? Also keep track of where the "off"
// should be placed?
// TODO: clean up, difficult to read rn
pub fn modify_niri_config(file_path: &str, output_name: &str, output_should_be_on: bool) {
    let kdl_str = fs::read_to_string(file_path).expect("Error reading niri KDL file");
    let mut doc: KdlDocument = kdl_str.parse().expect("Failed to parse niri KDL document");
    let mut should_write = false;

    if let Some(output_node) = doc.nodes_mut().iter_mut().find(|node| {
        if node.name().to_string() != "output" {
            return false;
        }

        node.get(0).map_or(false, |arg| {
            arg.to_string().to_lowercase() == output_name.to_lowercase()
        })
    }) {
        let children_opt = output_node.children_mut();
        let mut off_node_index = None;

        if let Some(children_doc) = children_opt.as_mut() {
            for (i, child_node) in children_doc.nodes().iter().enumerate() {
                if child_node.name().to_string() == "off" {
                    off_node_index = Some(i);
                    break;
                }
            }
        }

        if output_should_be_on {
            if off_node_index.is_some() {
                if let Some(children_doc) = children_opt.as_mut() {
                    children_doc.nodes_mut().remove(off_node_index.unwrap());
                    should_write = true;
                }
            }
        } else if off_node_index.is_none() {
            let off_node = KdlNode::new("off");
            if let Some(children_doc) = children_opt {
                children_doc.nodes_mut().push(off_node);
            } else {
                // no children block yet
                let mut new_children_doc = KdlDocument::new();
                new_children_doc.nodes_mut().push(off_node);
                output_node.set_children(new_children_doc);
            }
            should_write = true;
        }
    }

    if should_write {
        fs::write(file_path, doc.to_string()).expect("Failed to write modified KDL to niri config file");
    }
}
