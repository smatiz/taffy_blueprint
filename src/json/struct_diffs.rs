use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

// struct StyleWrapper<'a>(&'a Style);
// impl<'a> StyleWrapper<'a> {
//     fn _print(diff: &Diff, depth: usize, f: &mut DebugStruct) {
//         match diff {
//             Diff::Added(_) => {}
//             Diff::Removed(_) => {}
//             Diff::Changed { old, new } => {
//                 let _ = f.field(&format!("{} {} => {}", " ".repeat(depth + 3), old, new), &1);
//             }
//             Diff::Children(btree_map) => {
//                 for (s, diff) in btree_map {
//                     let _ = f.field(&format!("{} {}:", "-".repeat(depth + 1), s), &1);
//                     Self::_print(diff, depth + 2, f);
//                 }
//             }
//         }
//     }
// }

// impl<'a> Debug for StyleWrapper<'a> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut fx = f.debug_struct("Style");
//         let default_style = Style::default();
//         let diff = Diff::new(&default_style, self.0);
//         Self::_print(&diff, 0, &mut fx);

//         fx.finish()
//         //     .field("style", self.0.)
//         //     .field("id", &self.id)
//         //     .field("children", &self.children)
//         //     .finish()
//     }
// }

#[derive(Debug)]
pub enum Diff {
    Added(Value),
    Removed(Value),
    Changed { old: Value, new: Value },
    Children(BTreeMap<String, Diff>),
}

impl Diff {
    pub fn new<T>(a: &T, b: &T) -> Self
    where
        T: Serialize,
    {
        let aj = serde_json::to_value(a).unwrap();
        let bj = serde_json::to_value(b).unwrap();
        Self::_diff_values(&aj, &bj).unwrap()
    }
    fn _print(&self, depth: usize) {
        match self {
            Diff::Added(value) => println!("Added {}", value),
            Diff::Removed(value) => println!("Removed {}", value),
            Diff::Changed { old, new } => println!("{} {} => {}", " ".repeat(depth + 3), old, new),
            Diff::Children(btree_map) => {
                for (s, d) in btree_map {
                    println!("{} {}:", "-".repeat(depth + 1), s);
                    d._print(depth + 2);
                }
            }
        }
    }
    pub fn print(&self) {
        self._print(0);
    }

    fn _diff_values(a: &Value, b: &Value) -> Option<Diff> {
        match (a, b) {
            // Both objects → recurse
            (Value::Object(ma), Value::Object(mb)) => {
                let mut changes = BTreeMap::new();

                // Keys in either map
                let keys: std::collections::BTreeSet<_> = ma.keys().chain(mb.keys()).collect();

                for key in keys {
                    match (ma.get(key), mb.get(key)) {
                        (Some(va), Some(vb)) => {
                            if let Some(d) = Self::_diff_values(va, vb) {
                                changes.insert(key.clone(), d);
                            }
                        }
                        (None, Some(vb)) => {
                            changes.insert(key.clone(), Diff::Added(vb.clone()));
                        }
                        (Some(va), None) => {
                            changes.insert(key.clone(), Diff::Removed(va.clone()));
                        }
                        _ => {}
                    }
                }

                if changes.is_empty() {
                    None
                } else {
                    Some(Diff::Children(changes))
                }
            }

            // Primitive values → compare directly
            _ => {
                if a != b {
                    Some(Diff::Changed {
                        old: a.clone(),
                        new: b.clone(),
                    })
                } else {
                    None
                }
            }
        }
    }
}
