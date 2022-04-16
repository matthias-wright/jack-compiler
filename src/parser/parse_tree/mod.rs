//! Defines the parse tree.
pub mod class;
pub mod subroutine;
pub mod statement;
pub mod expression;
pub mod var;

use super::parse_tree::class::ClassNode;

/// Defines the parse tree.
pub struct ParseTree {
    pub class_node: ClassNode
}

