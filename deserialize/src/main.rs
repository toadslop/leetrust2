use std::{cell::RefCell, rc::Rc};

fn main() {
    Test::test1();
    Test::test2();
    Test::test3();
    Test::test4();
    Test::test5();
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

    pub fn wrap(node: Self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(node))
    }
}

struct Codec;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(root) = root {
            let val = Codec::serialize_value(root.borrow().val);
            let right = self.serialize(root.as_ref().borrow().right.clone());
            let right = Codec::serialize_node(&right, Side::Right);
            let left = self.serialize(root.as_ref().borrow().left.clone());
            let left = Codec::serialize_node(&left, Side::Left);

            return format!("{{{}}}", [val, right, left].join(""));
        } else {
            String::from("")
        }
    }

    fn serialize_value(val: i32) -> String {
        format!("v{}", val)
    }

    fn serialize_node(val: &String, side: Side) -> String {
        if val.len() > 0 {
            match side {
                Side::Left => format!(",l{}", val),
                Side::Right => format!(",r{}", val),
            }
        } else {
            val.to_owned()
        }
    }

    fn parse_val(data: &str, start: usize, end: usize) -> i32 {
        let slice = &data[start..end];
        slice.parse::<i32>().unwrap()
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut stack: Vec<(Option<i32>, Option<TreeNode>, Option<TreeNode>, ParseTarget)> =
            Vec::new();
        let mut parsing_val = false;
        let mut parse_start: usize = 0;
        let mut parse_target = ParseTarget::Root;
        let stackref = &mut stack;

        for (idx, char) in data.chars().enumerate() {
            match char {
                '{' => stackref.push((None, None, None, parse_target)),
                'l' => parse_target = ParseTarget::Left,
                'r' => parse_target = ParseTarget::Right,
                '}' => {
                    if parsing_val == true {
                        parsing_val = false;

                        stackref.last_mut().and_then(|last| {
                            last.0 = Some(Codec::parse_val(&data, parse_start, idx));
                            Some(last)
                        });
                    }

                    if let Some(item) = stackref.pop() {
                        let (val, left, right, parse_target) = item;
                        let mut node = TreeNode::new(val.unwrap_or(0));
                        if let Some(left) = left {
                            node.left = Some(Rc::new(RefCell::new(left)));
                        }

                        if let Some(right) = right {
                            node.right = Some(Rc::new(RefCell::new(right)));
                        }

                        if stackref.len() == 0 {
                            return Some(Rc::new(RefCell::new(node)));
                        } else {
                            match parse_target {
                                ParseTarget::Left => {
                                    let idx = stackref.len();
                                    let prev = stackref.get_mut(idx - 1).unwrap();

                                    prev.1 = Some(node);
                                }
                                ParseTarget::Right => {
                                    let idx = stackref.len();
                                    let prev = stackref.get_mut(idx - 1).unwrap();

                                    prev.2 = Some(node);
                                }
                                ParseTarget::Root => todo!(),
                            }
                        };
                    };
                }
                'v' => {
                    parsing_val = true;
                    parse_start = idx + 1;
                }
                ',' => {
                    if parsing_val == true {
                        parsing_val = false;

                        stackref.last_mut().and_then(|last| {
                            last.0 = Some(Codec::parse_val(&data, parse_start, idx));
                            Some(last)
                        });
                    }
                }
                _ => (),
            };
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum ParseTarget {
    Left,
    Right,
    Root,
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left,
    Right,
}

struct Test;

impl Test {
    fn test1() {
        let mut node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);

        node3.right = Some(TreeNode::wrap(node5));
        node3.left = Some(TreeNode::wrap(node4));
        node1.left = Some(TreeNode::wrap(node2));
        node1.right = Some(TreeNode::wrap(node3));

        let node1 = TreeNode::wrap(node1);

        let codec = Codec::new();
        let stringified = codec.serialize(Some(node1.clone()));
        println!("{}", stringified);
        let deserialized = codec.deserialize(stringified);

        assert!(Self::double_traverse(&Some(node1), &deserialized));
        println!("TEST 1 PASSED");
    }

    fn test4() {
        let mut node1 = TreeNode::new(11);
        let node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(35);
        let node4 = TreeNode::new(0);
        let node5 = TreeNode::new(522);

        node3.right = Some(TreeNode::wrap(node5));
        node3.left = Some(TreeNode::wrap(node4));
        node1.left = Some(TreeNode::wrap(node2));
        node1.right = Some(TreeNode::wrap(node3));

        let node1 = TreeNode::wrap(node1);

        let codec = Codec::new();
        let stringified = codec.serialize(Some(node1.clone()));
        println!("{}", stringified);
        let deserialized = codec.deserialize(stringified);

        assert!(Self::double_traverse(&Some(node1), &deserialized));
        println!("TEST 4 PASSED");
    }

    fn test5() {
        let mut node1 = TreeNode::new(11);
        let node2 = TreeNode::new(-2);
        let mut node3 = TreeNode::new(35);
        let node4 = TreeNode::new(0);
        let node5 = TreeNode::new(-522);

        node3.right = Some(TreeNode::wrap(node5));
        node3.left = Some(TreeNode::wrap(node4));
        node1.left = Some(TreeNode::wrap(node2));
        node1.right = Some(TreeNode::wrap(node3));

        let node1 = TreeNode::wrap(node1);

        let codec = Codec::new();
        let stringified = codec.serialize(Some(node1.clone()));
        println!("{}", stringified);
        let deserialized = codec.deserialize(stringified);

        assert!(Self::double_traverse(&Some(node1), &deserialized));
        println!("TEST 5 PASSED");
    }

    fn test2() {
        let node1 = TreeNode::new(1);

        let node1 = TreeNode::wrap(node1);

        let codec = Codec::new();
        let stringified = codec.serialize(Some(node1.clone()));
        let deserialized = codec.deserialize(stringified);

        assert!(Self::double_traverse(&Some(node1), &deserialized));
        println!("TEST 2 PASSED");
    }

    fn test3() {
        let mut node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);

        node1.left = Some(TreeNode::wrap(node2));
        let node1 = TreeNode::wrap(node1);

        let codec = Codec::new();
        let stringified = codec.serialize(Some(node1.clone()));
        println!("STRINIFIED:");
        println!("{}", stringified);
        let deserialized = codec.deserialize(stringified);

        assert!(Self::double_traverse(&Some(node1), &deserialized));
        println!("TEST 3 PASSED");
    }

    fn double_traverse(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1.as_ref(), root2.as_ref()) {
            (None, None) => true,
            (Some(root1), Some(root2)) => {
                if root1.as_ref().borrow().val != root2.as_ref().borrow().val {
                    return false;
                } else {
                    return Self::double_traverse(
                        &root1.as_ref().borrow().left,
                        &root2.as_ref().borrow().left,
                    ) && Self::double_traverse(
                        &root1.as_ref().borrow().right,
                        &root2.as_ref().borrow().right,
                    );
                }
            }
            _ => false,
        }
    }
}
