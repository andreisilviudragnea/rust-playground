use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct TreeNode {
    val: u32,
    left: RefCell<Option<Rc<TreeNode>>>,
    right: RefCell<Option<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(val: u32) -> TreeNode {
        TreeNode { val, left: RefCell::new(None), right: RefCell::new(None) }
    }

    fn from(input: &str) -> Rc<TreeNode> {
        let mut input = input.trim().trim_left_matches('[').trim_right_matches(']').split(',');

        let root = match input.next() {
            Some(val) => Rc::new(TreeNode::new(val.parse().unwrap())),
            None => panic!("no first value")
        };

        let mut queue: VecDeque<Rc<TreeNode>> = VecDeque::new();
        queue.push_back(Rc::clone(&root));

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            match input.next() {
                Some(val) => {
                    match val.trim() {
                        "null" => {}
                        val => {
                            let left = Rc::new(TreeNode::new(val.parse().unwrap()));
                            queue.push_back(left.clone());
                            *node.left.borrow_mut() = Some(left);
                        }
                    }
                }
                None => break
            }

            match input.next() {
                Some(val) => {
                    match val.trim() {
                        "null" => {}
                        val => {
                            let right = Rc::new(TreeNode::new(val.parse().unwrap()));
                            queue.push_back(right.clone());
                            *node.right.borrow_mut() = Some(right);
                        }
                    }
                }
                None => panic!("missing value")
            }
        }

        return root;
    }

    fn tree_to_string(tree: Rc<TreeNode>) -> String {
        let mut output = String::new();

        let mut queue: VecDeque<Option<Rc<TreeNode>>> = VecDeque::new();
        queue.push_back(Some(tree.clone()));

        while !queue.is_empty() {
            match queue.pop_front().unwrap() {
                Some(val) => {
                    output.push_str(&val.val.to_string());
                    queue.push_back(val.left.borrow().clone());
                    queue.push_back(val.right.borrow().clone());
                }
                None => { output.push_str("null") }
            }
            output.push_str(", ");
        }

        return format!("[{}]", output.trim_right_matches(", "));
    }
}

fn main() {
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[5, 4, 6, 2, null, null, 7, null, null, null, null]")),
               "[5, 4, 6, 2, null, null, 7, null, null, null, null]");
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[1, null, 2, null, null]")), "[1, null, 2, null, null]");
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[0, null, null]")), "[0, null, null]")
}
