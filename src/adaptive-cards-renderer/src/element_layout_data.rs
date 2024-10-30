use taffy::NodeId;

use crate::rect::FinalRect;

/// The layout data for an element. This is used to store transient layout data on an element
/// struct during the layout and draw passes of the renderer.
#[derive(serde::Serialize, Default, Debug, Clone)]
pub struct ElementLayoutData {
    /// The final absolute rect of the element which was used to draw the element.
    pub rect: Option<FinalRect>,

    /// The taffy data for the element. This is used to store the node id and child element node ids
    /// for the element during the layout pass, so they can be later referenced during the draw pass.
    #[serde(skip)]
    pub taffy_data: Option<ElementTaffyData>,

    /// The placement of the element relative to its sibling elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<Placement>,
}

impl ElementLayoutData {
    /// Returns the placement of the element relative to its sibling elements,
    /// or panics if the placement is not set.
    pub fn placement(&self) -> Placement {
        self.placement.expect("Element placement should be set")
    }
}

/// The taffy data for an element. This is used to store the node id and child element node ids
/// for the element during the layout pass, so they can be later referenced during the draw pass.
#[derive(Debug, Clone)]
pub struct ElementTaffyData {
    /// The Taffy node id of the element.
    pub node_id: NodeId,

    /// The Taffy node ids of the child elements of the element.
    /// This is useful for contains which contain child elements, but also decorative children
    /// such as spacers or separators.
    /// By using this field the draw pass can find the child nodes representing child elements and
    /// ignore the decorative children.
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

/// The placement of an element relative to its sibling elements.
#[derive(serde::Serialize, Debug, Copy, Clone)]
pub enum Placement {
    /// The element is the top element in a vertical layout.
    Top,

    /// The element is the bottom element in a vertical layout.
    Bottom,

    /// The element is the sole element in a vertical layout
    /// (it is both the top-most and bottom-most element).
    SoleVertical,

    /// The element is within a vertical layout but is neither the top-most
    /// nor the bottom-most element.
    WithinVertical,

    /// The element is the left element in a horizontal layout.
    Left,

    /// The element is the right element in a horizontal layout.
    Right,

    /// The element is the sole element in a horizontal layout
    /// (it is both the left-most and right-most element).
    SoleHorizontal,

    /// The element is within a horizontal layout, but is neither the left-most
    /// nor the right-most element.
    WithinHorizontal,
}
