use taffy::NodeId;

use crate::rect::FinalRect;

#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct ElementLayoutData {
    pub rect: Option<FinalRect>,

    #[serde(skip)]
    pub taffy_data: Option<ElementTaffyData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_within_parent: Option<PositionWithinParent>,
}

#[derive(Debug, Clone)]
pub struct ElementTaffyData {
    pub node_id: NodeId,
    pub child_element_node_ids: Vec<NodeId>,
}

impl From<NodeId> for ElementTaffyData {
    fn from(value: NodeId) -> Self {
        ElementTaffyData {
            node_id: value,
            child_element_node_ids: Vec::new(),
        }
    }
}

#[derive(serde::Serialize, Debug, Copy, Clone)]
pub enum PositionWithinParent {
    Top,
    Bottom,
    VerticalOnly,
    VerticalMiddle,
    Left,
    Right,
    HorizontalOnly,
    HorizontalMiddle,
}
