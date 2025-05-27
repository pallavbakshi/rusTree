// src/core/sorter.rs
use crate::core::node::NodeInfo;
use std::cmp::Ordering;


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Size,
    MTime,
    Words,
    Lines,
    Custom,
}

pub fn sort_nodes(nodes: &mut Vec<NodeInfo>, key: &SortKey, reverse: bool) {
    match key {
        SortKey::Name => {
            nodes.sort_by(|a, b| a.name.cmp(&b.name));
        }
        SortKey::Size => {
            nodes.sort_by(|a, b| {
                // Treat None as smallest for ascending, largest for descending
                // Or consistently: None comes after Some when ascending.
                match (a.size, b.size) {
                    (Some(sa), Some(sb)) => sa.cmp(&sb),
                    (Some(_), None) => Ordering::Less,    // Some is "smaller" than None (comes first)
                    (None, Some(_)) => Ordering::Greater, // None is "larger" than Some (comes last)
                    (None, None) => Ordering::Equal,
                }
            });
        }
        SortKey::MTime => {
            // Placeholder: Implement MTime sort
            // nodes.sort_by(|a, b| a.mtime.cmp(&b.mtime)); // Needs careful None handling
        }
        SortKey::Words => {
            // Placeholder: Implement Words sort
            // nodes.sort_by(|a, b| a.word_count.cmp(&b.word_count)); // Needs careful None handling
        }
        SortKey::Lines => {
            // Placeholder: Implement Lines sort
            // nodes.sort_by(|a, b| a.line_count.cmp(&b.line_count)); // Needs careful None handling
        }
        SortKey::Custom => {
            // Placeholder: Implement Custom sort
            // nodes.sort_by(|a, b| {
            //    // Logic for comparing custom_function_output, which is Option<Result<String, _>>
            // });
        }
    }

    if reverse {
        nodes.reverse();
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::node::NodeType;
    use std::path::PathBuf;
    use std::time::SystemTime;

    fn create_test_node(name_str: &str, size: Option<u64>, line_count: Option<usize>) -> NodeInfo {
        NodeInfo {
            path: PathBuf::from(name_str),
            name: name_str.to_string(),
            node_type: NodeType::File, // Simplified for test
            depth: 0,
            size,
            permissions: None,
            mtime: Some(SystemTime::now()),
            line_count,
            word_count: None,
            custom_function_output: None, // This is Option<Result<String, ApplyFnError>>
        }
    }

    #[test]
    fn test_sort_by_name() {
        let mut nodes = vec![
            create_test_node("charlie.txt", Some(100), Some(10)),
            create_test_node("alpha.txt", Some(200), Some(5)),
            create_test_node("beta.txt", Some(50), Some(20)),
        ];
        sort_nodes(&mut nodes, &SortKey::Name, false);
        assert_eq!(nodes[0].name, "alpha.txt");
        assert_eq!(nodes[1].name, "beta.txt");
        assert_eq!(nodes[2].name, "charlie.txt");
    }

    #[test]
    fn test_sort_by_name_reverse() {
        let mut nodes = vec![
            create_test_node("charlie.txt", Some(100), Some(10)),
            create_test_node("alpha.txt", Some(200), Some(5)),
            create_test_node("beta.txt", Some(50), Some(20)),
        ];
        sort_nodes(&mut nodes, &SortKey::Name, true);
        assert_eq!(nodes[0].name, "charlie.txt");
        assert_eq!(nodes[1].name, "beta.txt");
        assert_eq!(nodes[2].name, "alpha.txt");
    }

    #[test]
    fn test_sort_by_size_handles_none() {
        let mut nodes = vec![
            create_test_node("big.txt", Some(1000), None),
            create_test_node("unknown.txt", None, None),
            create_test_node("small.txt", Some(10), None),
        ];
        
        // Current sort_nodes for Size: Some < None (Some comes first)
        sort_nodes(&mut nodes, &SortKey::Size, false); // Ascending
        assert_eq!(nodes[0].name, "small.txt");     // Some(10)
        assert_eq!(nodes[1].name, "big.txt");       // Some(1000)
        assert_eq!(nodes[2].name, "unknown.txt");   // None

        // Re-sort for reverse
        let mut nodes_rev = vec![
            create_test_node("big.txt", Some(1000), None),
            create_test_node("unknown.txt", None, None),
            create_test_node("small.txt", Some(10), None),
        ];
        sort_nodes(&mut nodes_rev, &SortKey::Size, true); // Descending
        assert_eq!(nodes_rev[0].name, "unknown.txt"); // None (comes first when reversed because it was last)
        assert_eq!(nodes_rev[1].name, "big.txt");     // Some(1000)
        assert_eq!(nodes_rev[2].name, "small.txt");   // Some(10)
    }
}