use taffy::TaffyError;

#[derive(Debug, PartialEq)]
pub enum TaffyBlueprintError {
    Taffy(TaffyError),
    TaffyNodeInner,
    TaffyNodeRaw,
    Json(String),
    Prune(String),
}
