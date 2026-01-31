use crate::{core::TaffyBlueprintError, json::NodeJsonError};

pub enum PruneResult {
    Replace,
    Keep,
    Undefined,
}

pub trait Prune
where
    Self: Sized,
{
    fn keep(&self) -> PruneResult;
    fn children(&mut self) -> Vec<Self>;
    fn replace_children(self, children: Vec<Self>) -> Self;
}
fn converted_children<N: Prune>(node: &mut N) -> Result<Vec<N>, TaffyBlueprintError> {
    node.children()
        .into_iter()
        .map(|c| _prune_tree(c))
        .collect::<Result<Vec<_>, _>>()
        .map(|v| v.into_iter().flatten().collect())
}

fn _prune_tree<N: Prune>(mut node: N) -> Result<Vec<N>, TaffyBlueprintError> {
    match node.keep() {
        PruneResult::Replace => converted_children(&mut node),
        PruneResult::Keep => match converted_children(&mut node) {
            Ok(converted_children) => Ok(vec![node.replace_children(converted_children)]),
            Err(e) => Err(e),
        },
        PruneResult::Undefined => Err(TaffyBlueprintError::Prune),
    }
}

pub fn prune_tree<N>(root: N) -> Result<N, TaffyBlueprintError>
where
    N: Prune + std::fmt::Debug,
{
    let pruned_tree = _prune_tree(root);
    match pruned_tree {
        // TODO remove unwrap
        Ok(pruned_tree) => Ok(pruned_tree.into_iter().next().unwrap()),
        Err(e) => Err(e),
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
        fn keep(&self) -> bool {
            self.id.is_some()
        }

        fn children(&mut self) -> Vec<Self> {
            self.children.drain(0..self.children.len()).collect()
        }

        fn replace_children(self, children: Vec<Self>) -> Self {
            Self { children, ..self }
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
            None,
        );

        assert_eq!(
            prune_tree(A {
                id: Some("1".to_string()),
                children: vec![],
            }),
            Some(A {
                id: Some("1".to_string()),
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
            Some(A {
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
            Some(A {
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
