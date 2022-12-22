// Nodes (for the program's AST).

/// Conditional
struct ConditionalNode {
    condition: Vec<Node>
}

impl ConditionalNode {
    pub fn new(condition: Vec<Node>) -> ConditionalNode {
        return ConditionalNode {
            condition
        };
    }
}

/// The actual Node enum, for grouping all types of nodes in a single iterator to walk.
enum Node {

}