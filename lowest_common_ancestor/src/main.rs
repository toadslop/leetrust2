fn main() {
    let root = initialize_tree();
    Tests::test1(root.clone());
    Tests::test2(root.clone());
    Tests::test3(root.clone());
    Tests::test4(root.clone());
}

fn initialize_tree() -> Rc<RefCell<TreeNode>> {
    let mut node3 = TreeNode::new(3);
    let mut node5 = TreeNode::new(5);
    let mut node1 = TreeNode::new(1);
    let node6 = TreeNode::new(6);
    let mut node2 = TreeNode::new(2);
    let node0 = TreeNode::new(0);
    let node8 = TreeNode::new(8);
    let node7 = TreeNode::new(7);
    let node4 = TreeNode::new(4);

    node2.left = Some(wrap_node(node7));
    node2.right = Some(wrap_node(node4));
    node5.left = Some(wrap_node(node6));
    node5.right = Some(wrap_node(node2));
    node1.left = Some(wrap_node(node0));
    node1.right = Some(wrap_node(node8));
    node3.left = Some(wrap_node(node5));
    node3.right = Some(wrap_node(node1));
    wrap_node(node3)
}

fn wrap_node(node: TreeNode) -> Rc<RefCell<TreeNode>> {
    Rc::new(RefCell::new(node))
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
        if let (Some(root), Some(p), Some(q)) = (root, p, q) {
            Solution::find(root, &p, &q)
        } else {
            None
        }
    }

    fn find(
        root: Rc<RefCell<TreeNode>>,
        p: &Rc<RefCell<TreeNode>>,
        q: &Rc<RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if p.borrow().val == root.borrow().val {
            return Some(p.clone());
        };

        if q.borrow().val == root.borrow().val {
            return Some(q.clone());
        };

        let left = if let Some(left) = &root.borrow().left {
            Solution::find(left.clone(), p, q)
        } else {
            None
        };

        let right = if let Some(right) = &root.borrow().right {
            Solution::find(right.clone(), p, q)
        } else {
            None
        };

        match (left.as_ref(), right.as_ref()) {
            (None, None) => None,
            (None, Some(_)) => right,
            (Some(_), None) => left,
            (Some(_), Some(_)) => Some(root),
        }
    }
}

struct Tests;
impl Tests {
    pub fn test1(root: Rc<RefCell<TreeNode>>) {
        let p = wrap_node(TreeNode::new(5));
        let q = wrap_node(TreeNode::new(1));
        let result = Solution::lowest_common_ancestor(Some(root), Some(p), Some(q));
        let result = result.unwrap().as_ref().borrow().val;
        assert_eq!(result, 3);
        println!("TEST SUCCEEDED");
    }

    pub fn test2(root: Rc<RefCell<TreeNode>>) {
        let p = wrap_node(TreeNode::new(5));
        let q = wrap_node(TreeNode::new(4));
        let result = Solution::lowest_common_ancestor(Some(root), Some(p), Some(q));
        let result = result.unwrap().as_ref().borrow().val;
        assert_eq!(result, 5);
        println!("TEST SUCCEEDED");
    }

    pub fn test3(root: Rc<RefCell<TreeNode>>) {
        let p = wrap_node(TreeNode::new(7));
        let q = wrap_node(TreeNode::new(8));
        let result = Solution::lowest_common_ancestor(Some(root), Some(p), Some(q));
        let result = result.unwrap().as_ref().borrow().val;
        assert_eq!(result, 3);
        println!("TEST SUCCEEDED");
    }

    pub fn test4(root: Rc<RefCell<TreeNode>>) {
        let p = wrap_node(TreeNode::new(6));
        let q = wrap_node(TreeNode::new(4));
        let result = Solution::lowest_common_ancestor(Some(root), Some(p), Some(q));
        let result = result.unwrap().as_ref().borrow().val;
        assert_eq!(result, 5);
        println!("TEST SUCCEEDED");
    }
}
