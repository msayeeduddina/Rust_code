fn check_box_int() {
    // Allocate an integer on the heap using Box
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1); // prints 10
}

fn check_tree_node() {
    // Define a generic binary-tree node
    struct TreeNode<T> {
        pub left: Option < Box < TreeNode<T>>> , // optional left child
        pub right: Option < Box < TreeNode<T>>> , // optional right child
        pub value: T, // payload
    }

    impl <T> TreeNode<T> {
        // Constructor: create a leaf node with the given value
        pub fn new(value: T) -> Self {
            TreeNode {
                left: None,
                right: None,
                value
            }
        }

        // Consume self, attach a left child, return self for chaining
        pub fn left(mut self, node: TreeNode<T> ) -> Self {
            self.left = Some(Box::new(node));
            self
        }
        // Consume self, attach a right child, return self for chaining
        pub fn right(mut self, node: TreeNode<T> ) -> Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    // Build a tiny tree: root(1) with left(2) and right(3)
    let _node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));

    // Print the root value
    println!("TreeNode with value {} created.", _node1.value);
    // Print the left child's value (unwrap the Box inside Option)
    println!(
        "TreeNode with left value {} created.",
        _node1.left.as_ref().unwrap().value
    );
    // Print the right child's value (unwrap the Box inside Option)
    println!(
        "TreeNode with right value {} created.",
        _node1.right.as_ref().unwrap().value
    );
}

fn main() {
    check_box_int(); // demo simple Box usage
    check_tree_node(); // demo boxed tree construction
}