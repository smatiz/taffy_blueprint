pub trait Prune
where
    Self: Sized,
{
    type Id;
    type Output;

    fn keep(&self) -> bool;
    fn children(&mut self) -> Vec<Self>;
    fn make_output(node: Self, children: Vec<Self::Output>) -> Self::Output;
}

fn _prune_tree<N: Prune>(mut node: N) -> Vec<N::Output> {
    let keep = node.keep();
    let converted_children: Vec<N::Output> = node
        .children()
        .into_iter()
        .flat_map(|c| _prune_tree(c))
        .collect();
    if keep {
        vec![N::make_output(node, converted_children)]
    } else {
        converted_children
    }
}

pub fn prune_tree<N>(root: N) -> Option<N::Output>
where
    N: Prune,
{
    if root.keep() {
        let pruned_tree = _prune_tree(root);
        if pruned_tree.len() == 1 {
            pruned_tree.into_iter().next()
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq)]
    struct A {
        id: Option<String>,
        children: Vec<A>,
    }
    impl Prune for A {
        type Id = String;
        type Output = B;

        fn keep(&self) -> bool {
            self.id.is_some()
        }

        fn children(&mut self) -> Vec<Self> {
            self.children.drain(0..self.children.len()).collect()
        }

        fn make_output(n: Self, children: Vec<Self::Output>) -> Self::Output {
            Self::Output {
                id: n.id.unwrap(),
                children,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    struct B {
        id: String,
        children: Vec<B>,
    }

    use super::*;
    #[test]
    fn test_prune() {
        assert_eq!(
            prune_tree(A {
                id: None,
                children: vec![],
            }),
            None,
        );

        assert_eq!(
            prune_tree(A {
                id: Some("1".to_string()),
                children: vec![],
            }),
            Some(B {
                id: "1".to_string(),
                children: vec![],
            }),
        );

        assert_eq!(
            prune_tree(A {
                id: Some("ROOT".to_string()),
                children: vec![A {
                    id: None,
                    children: vec![A {
                        id: None,
                        children: vec![A {
                            id: None,
                            children: vec![A {
                                id: Some("1".to_string()),
                                children: vec![]
                            }],
                        }],
                    }],
                }],
            }),
            Some(B {
                id: "ROOT".to_string(),
                children: vec![B {
                    id: "1".to_string(),
                    children: vec![],
                }]
            }),
        );

        assert_eq!(
            prune_tree(A {
                id: Some("ROOT".to_string()),
                children: vec![A {
                    id: None,
                    children: vec![
                        A {
                            id: None,
                            children: vec![A {
                                id: None,
                                children: vec![A {
                                    id: Some("1".to_string()),
                                    children: vec![]
                                }],
                            }],
                        },
                        A {
                            id: None,
                            children: vec![A {
                                id: None,
                                children: vec![A {
                                    id: Some("2".to_string()),
                                    children: vec![
                                        A {
                                            id: Some("3".to_string()),
                                            children: vec![]
                                        },
                                        A {
                                            id: None,
                                            children: vec![A {
                                                id: None,
                                                children: vec![A {
                                                    id: Some("5".to_string()),
                                                    children: vec![
                                                        A {
                                                            id: Some("6".to_string()),
                                                            children: vec![]
                                                        },
                                                        A {
                                                            id: None,
                                                            children: vec![]
                                                        }
                                                    ]
                                                }],
                                            }]
                                        },
                                        A {
                                            id: Some("4".to_string()),
                                            children: vec![]
                                        }
                                    ]
                                }],
                            }],
                        }
                    ],
                }],
            }),
            Some(B {
                id: "ROOT".to_string(),
                children: vec![
                    B {
                        id: "1".to_string(),
                        children: vec![],
                    },
                    B {
                        id: "2".to_string(),
                        children: vec![
                            B {
                                id: "3".to_string(),
                                children: vec![],
                            },
                            B {
                                id: "5".to_string(),
                                children: vec![B {
                                    id: "6".to_string(),
                                    children: vec![],
                                }],
                            },
                            B {
                                id: "4".to_string(),
                                children: vec![],
                            },
                        ],
                    }
                ]
            }),
        );
    }
}
