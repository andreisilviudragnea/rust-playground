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
}

fn string_to_treenode(input: &str) -> Rc<TreeNode> {
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
            Some("null") => {}
            Some(val) => {
                let left = Rc::new(TreeNode::new(val.parse().unwrap()));
                queue.push_back(left.clone());
                *node.left.borrow_mut() = Some(left);
            }
            None => break
        }

        match input.next() {
            Some("null") => {}
            Some(val) => {
                let right = Rc::new(TreeNode::new(val.parse().unwrap()));
                queue.push_back(right.clone());
                *node.right.borrow_mut() = Some(right);
            }
            None => panic!("missing value")
        }
    }

    return root;
}

fn main() {
    let tree = string_to_treenode("[5,3,6,2,4,null,7]");
    println!("{:?}", tree);
}
