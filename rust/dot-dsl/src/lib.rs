macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            attrs.iter().for_each(|x| {
                self.attrs.insert(x.0.to_string(), x.1.to_string());
            });
            self
        }
        pub fn get_attr(&self, name: &str) -> Option<&str> {
            self.attrs.get(name).map(|x| x.as_str())
        }
    };
}

pub mod graph {
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;
    use std::collections::HashMap;

    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            edges.iter().for_each(|x| self.edges.push(x.clone()));
            self
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            nodes.iter().for_each(|x| self.nodes.push(x.clone()));
            self
        }

        pub fn get_node(&self, node_name: &str) -> Option<&Node> {
            self.nodes.iter().find(|&x| x.name.as_str() == node_name)
        }

        impl_attrs!();
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    Self {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                impl_attrs!();
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node_name: &str) -> Self {
                    Self {
                        name: node_name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                impl_attrs!();
            }
        }
    }
}
