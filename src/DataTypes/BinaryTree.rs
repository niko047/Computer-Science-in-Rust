
/* Binary tree, definition:
In computer science, a binary tree is a tree data structure in which each node has at most two children,
which are referred to as the left child and the right child
 */


enum Side {
    Right,
    Left,
}

#[derive(Debug)]
struct TreeNode<T> {
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
    value: T,
}

impl<T> TreeNode<T> {
    fn new(value:T) -> Self {
        TreeNode {
            left: None,
            right: None,
            value,
        }
    }

    fn add(&mut self, tree: Box<TreeNode<T>>, side: Side) {
        match side {
            Side::Right => {self.right = Some(tree);},
            Side::Left=> {self.left = Some(tree);},
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binarytree() {
        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(10);
        let mut t3 = TreeNode::new(7);
        t1.add(Box::new(t2), Side::Left);
        t1.add(Box::new(t3), Side::Right);
        println!("{:?}", &t1);
        assert_eq!(t1.left.unwrap().value, 10);
        assert_eq!(t1.right.unwrap().value, 7);
    }
}