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

    fn from(input: &str) -> Option<Box<TreeNode>> {
        let mut input = input.trim().trim_left_matches('[').trim_right_matches(']').split(',');

        let mut root = match input.next() {
            Some(val) => Some(Box::new(TreeNode::new(val.parse().unwrap()))),
            None => panic!("no first value")
        };

        {
            let mut queue: VecDeque<&mut TreeNode> = VecDeque::new();
            let r = root.as_mut().unwrap();
            queue.push_back(r);

            while !queue.is_empty() {
                let node = queue.pop_front().unwrap();

                match input.next() {
                    Some(val) => {
                        match val.trim() {
                            "null" => {}
                            val => {
                                node.left = Some(Box::new(TreeNode::new(val.parse().unwrap())));
                                queue.push_back(node.left.as_mut().unwrap());
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
                                node.right = Some(Box::new(TreeNode::new(val.parse().unwrap())));
                                queue.push_back(node.right.as_mut().unwrap());
                            }
                        }
                    }
                    None => panic!("missing value")
                }
            }
        }

        return root;
    }

    fn tree_to_string(tree: &Option<Box<TreeNode>>) -> String {
        let mut output = String::new();

        let mut queue: VecDeque<&Option<Box<TreeNode>>> = VecDeque::new();
        queue.push_back(tree);

        while !queue.is_empty() {
            match *queue.pop_front().unwrap() {
                Some(ref val) => {
                    output.push_str(&val.val.to_string());
                    queue.push_back(&val.left);
                    queue.push_back(&val.right);
                }
                None => { output.push_str("null") }
            }
            output.push_str(", ");
        }

        return format!("[{}]", output.trim_right_matches(", "));
    }

    fn find<'a>(parent: &'a Option<Box<TreeNode>>, root: &'a Option<Box<TreeNode>>, key: u32)
                -> (&'a Option<Box<TreeNode>>, &'a Option<Box<TreeNode>>) {
        if let Some(ref the_root) = *root {
            if key < the_root.val {
                return TreeNode::find(root, &the_root.left, key);
            }
            if key > the_root.val {
                return TreeNode::find(root, &the_root.right, key);
            }
        }
        return (root, parent);
    }
}

fn main() {
    assert_eq!(TreeNode::tree_to_string(&TreeNode::from("[5, 4, 6, 2, null, null, 7, null, null, null, null]")),
               "[5, 4, 6, 2, null, null, 7, null, null, null, null]");
    assert_eq!(TreeNode::tree_to_string(&TreeNode::from("[1, null, 2, null, null]")), "[1, null, 2, null, null]");
    assert_eq!(TreeNode::tree_to_string(&TreeNode::from("[0, null, null]")), "[0, null, null]");
    let tree = TreeNode::from("[5, 4, 6, 2, null, null, 7, null, null, null, null]");
    let ret = TreeNode::find(&None, &tree, 7);
    println!("{:?}", ret);
}
