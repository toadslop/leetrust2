fn main() {
    let root = initialize_tree();
    Tests::test1(root);
}

fn initialize_tree() -> Option<Rc<RefCell<TreeNode>>> {
    let mut node3 = TreeNode::new(3);
    let mut node5 = TreeNode::new(5);
    let mut node1 = TreeNode::new(1);
    let node6 = TreeNode::new(6);
    let mut node2 = TreeNode::new(2);
    let node0 = TreeNode::new(0);
    let node8 = TreeNode::new(8);
    let node7 = TreeNode::new(7);
    let node4 = TreeNode::new(4);

    node2.left = wrap_node(node7);
    node2.right = wrap_node(node4);
    node5.left = wrap_node(node6);
    node5.right = wrap_node(node2);
    node1.left = wrap_node(node0);
    node1.right = wrap_node(node8);
    node3.left = wrap_node(node5);
    node3.right = wrap_node(node1);
    wrap_node(node3)
}

fn wrap_node(node: TreeNode) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(node)))
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        unimplemented!()
    }
}

struct Tests;
impl Tests {
    pub fn test1(root: Option<Rc<RefCell<TreeNode>>>) {
        let p = wrap_node(TreeNode::new(5));
        let q = wrap_node(TreeNode::new(1));
        Solution::lowest_common_ancestor(root, p, q);
    }
}
