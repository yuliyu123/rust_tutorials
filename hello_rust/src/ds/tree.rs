use std::cmp::Ordering;

pub struct TreeNode {
    val: i32,
    left: TreeNodeLink,
    right: TreeNodeLink,
}

type TreeNodeLink = Option<Box<TreeNode>>;

pub fn invert_tree(root: &mut TreeNode) {
    let (mut tmp_left, mut tmp_right) = (TreeNodeLink::None, TreeNodeLink::None);

    if let Some(mut node) = root.left.take() {
        invert_tree(&mut node);
        tmp_left = Some(node);
    }
    if let Some(mut node) = root.right.take() {
        invert_tree(&mut node);
        tmp_right = Some(node);
    }

    root.left = tmp_left;
    root.right = tmp_right;
}


#[test]
fn tree_test() {
    let mut root = TreeNode {
        val: 0,
        left: None,
        right: None,
    };
    let left = TreeNode {
        val: 1,
        left: None,
        right: None,
    };
    let right = TreeNode {
        val: 2,
        left: None,
        right: None,
    };
    root.left = Some(Box::new(left));
    root.right = Some(Box::new(right));
    invert_tree(&mut root);

    println!("{:?}", root.val);
    if let Some(ref x) = root.left {
        println!("{:?}", x.val);
    }
    if let Some(ref x) = root.right {
        println!("{:?}", x.val);
    }
}
