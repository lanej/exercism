pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            return Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            };
        }

        pub fn get_node(self, name: &str) -> Option<Node> {
            return Some(
                self.nodes
                    .iter()
                    .filter(|n| n.value == name)
                    .next()
                    .unwrap()
                    .clone(),
            );
        }

        pub fn with_nodes(mut self, nodes: &Vec<graph_items::node::Node>) -> Self {
            self.nodes = nodes.to_vec();
            return self;
        }

        pub fn with_edges(mut self, edges: &Vec<graph_items::edge::Edge>) -> Self {
            self.edges = edges.to_vec();
            return self;
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }

            return self;
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge {
                x: String,
                y: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(x: &str, y: &str) -> Self {
                    return Edge {
                        attrs: HashMap::new(),
                        x: x.to_owned(),
                        y: y.to_owned(),
                    };
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    return self;
                }
            }
        }
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node {
                pub value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }

                    return self;
                }

                pub fn get_attr(self, name: &str) -> Option<String> {
                    return Some(self.attrs.get(name).unwrap().clone());
                }
            }
        }
    }
}
