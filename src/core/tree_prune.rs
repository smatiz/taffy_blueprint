use crate::core::TaffyBlueprintError;

pub trait Prune
where
    Self: Sized,
{
    fn keep(&self) -> bool;
    fn children(&self) -> &Vec<Self>;
    fn create_with(&self, children: Vec<Self>) -> Self;
}
fn converted_children<N: Prune>(node: &N) -> Vec<N> {
    node.children()
        .iter()
        .flat_map(|c| _prune_tree(c))
        .collect()
}

fn _prune_tree<N: Prune>(node: &N) -> Vec<N> {
    if node.keep() {
        vec![node.create_with(converted_children(node))]
    } else {
        converted_children(node)
    }
}

pub fn prune_tree<N>(root: N) -> Result<N, TaffyBlueprintError>
where
    N: Prune + std::fmt::Debug,
{
    if root.keep() {
        _prune_tree(&root)
            .into_iter()
            .next()
            .ok_or(TaffyBlueprintError::Prune("?".into()))
    } else {
        Err(TaffyBlueprintError::Prune("Root needs an id".into()))
    }
}

#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq, Clone)]
    struct A {
        id: Option<String>,
        children: Vec<A>,
    }

    impl Prune for A {
        fn keep(&self) -> bool {
            self.id.is_some()
        }

        fn children<'a>(&'a self) -> &'a Vec<Self> {
            &self.children
        }

        fn create_with(&self, children: Vec<Self>) -> Self {
            Self {
                children,
                ..self.clone()
            }
        }
    }

    use super::*;
    #[test]
    fn test_prune() {
        assert_eq!(
            prune_tree(A {
                id: None,
                children: vec![],
            }),
            Err(TaffyBlueprintError::Prune("Root needs an id".into()))
        );

        assert_eq!(
            prune_tree(A {
                id: Some("1".to_string()),
                children: vec![],
            }),
            Ok(A {
                id: Some("1".to_string()),
                children: vec![],
            })
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
            Ok(A {
                id: Some("ROOT".to_string()),
                children: vec![A {
                    id: Some("1".to_string()),
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
            Ok(A {
                id: Some("ROOT".to_string()),
                children: vec![
                    A {
                        id: Some("1".to_string()),
                        children: vec![],
                    },
                    A {
                        id: Some("2".to_string()),
                        children: vec![
                            A {
                                id: Some("3".to_string()),
                                children: vec![],
                            },
                            A {
                                id: Some("5".to_string()),
                                children: vec![A {
                                    id: Some("6".to_string()),
                                    children: vec![],
                                }],
                            },
                            A {
                                id: Some("4".to_string()),
                                children: vec![],
                            },
                        ],
                    }
                ]
            }),
        );
    }
}
