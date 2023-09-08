use crate::ztype::array_traits::delta_context::DeltaContext;
/*
A packing context node is a node inside a packing context tree, which recursively
covers all zserio objects.
Each node may have children (one for each packable fields inside it).
For example, the packing context node of a struct would contain a packing context
node for each packable array/struct/choice field it contains as fields.
 */
// TODO Can this be a Enum type?
pub struct PackingContextNode {
    pub children: Vec<PackingContextNode>,
    pub context: Option<DeltaContext>,
}

impl PackingContextNode {
    pub fn new() -> PackingContextNode {
        PackingContextNode {
            children: vec![],
            context: None,
        }
    }
}

impl Default for PackingContextNode {
    fn default() -> Self {
        Self::new()
    }
}
