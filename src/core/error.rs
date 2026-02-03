use taffy::TaffyError;

#[derive(Debug, PartialEq)]
pub enum TaffyBlueprintError {
    Taffy(TaffyError),
    TaffyNodeInner,
    TaffyNodeRaw(String),
    Json(String),
    Prune(String),
}
