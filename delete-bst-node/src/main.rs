use std::collections::VecDeque;

#[derive(Debug)]
struct TreeNode {
    val: u32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: u32) -> TreeNode {
        TreeNode { val, left: None, right: None }
    }

    fn from(input: &str) -> Box<TreeNode> {
        let mut input = input.trim().trim_left_matches('[').trim_right_matches(']').split(',');

        let root = match input.next() {
            Some(val) => Box::new(TreeNode::new(val.parse().unwrap())),
            None => panic!("no first value")
        };

        {
            let mut queue: VecDeque<&TreeNode> = VecDeque::new();
            queue.push_back(&*root);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                match input.next() {
                    Some(val) => {
                        match val.trim() {
                            "null" => {}
                            val => {
                                let left = Box::new(TreeNode::new(val.parse().unwrap()));
                                unsafe { *(&node.left as *const Option<Box<TreeNode>> as *mut Option<Box<TreeNode>>) = Some(left); }
                                queue.push_back(node.left.as_ref().unwrap());
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
                                let right = Box::new(TreeNode::new(val.parse().unwrap()));
                                unsafe { *(&node.right as *const Option<Box<TreeNode>> as *mut Option<Box<TreeNode>>) = Some(right); }
                                queue.push_back(node.right.as_ref().unwrap());
                            }
                        }
                    }
                    None => panic!("missing value")
                }
            }
        }

        return root;
    }

    fn tree_to_string(tree: Box<TreeNode>) -> String {
        let mut output = String::new();

        let mut queue: VecDeque<Option<&TreeNode>> = VecDeque::new();
        queue.push_back(Some(&*tree));

        while !queue.is_empty() {
            match queue.pop_front().unwrap() {
                Some(val) => {
                    output.push_str(&val.val.to_string());
                    queue.push_back(match val.left {
                        Some(ref val) => Some(&**val),
                        None => None
                    });
                    queue.push_back(match val.right {
                        Some(ref val) => Some(&**val),
                        None => None
                    });
                }
                None => { output.push_str("null") }
            }
            output.push_str(", ");
        }

        return format!("[{}]", output.trim_right_matches(", "));
    }

    fn find<'a>(parent: &'a Option<Box<TreeNode>>, root: &'a Option<Box<TreeNode>>, key: u32)
        -> (&'a Option<Box<TreeNode>>, &'a Option<Box<TreeNode>>) {
        match *root {
            None => (&None, &None),
            Some(ref the_root) => {
                if key < the_root.val {
                    return TreeNode::find(root, &the_root.left, key);
                }
                if key > the_root.val {
                    return TreeNode::find(root, &the_root.right, key);
                }
                return (root, parent);
            }
        }
    }

    fn delete_node(tree: &mut TreeNode, key: u32) {

    }
}

fn main() {
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[5, 4, 6, 2, null, null, 7, null, null, null, null]")),
               "[5, 4, 6, 2, null, null, 7, null, null, null, null]");
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[1, null, 2, null, null]")), "[1, null, 2, null, null]");
    assert_eq!(TreeNode::tree_to_string(TreeNode::from("[0, null, null]")), "[0, null, null]");
    let tree = TreeNode::from("[5, 4, 6, 2, null, null, 7, null, null, null, null]");
    let some_tree = Some(tree);
    let ret = TreeNode::find(&None, &some_tree, 7);
    println!("{:?}", ret);
}
