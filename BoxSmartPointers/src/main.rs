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

    // `T` is Rust’s way of saying “I’ll decide the real type later.”  
    // It is a **generic type parameter**: a placeholder that lets you write one piece of code that the compiler can *monomorphize* (instantiate) for any concrete type you actually use.
    // When you write `T` you are promising:
    // 1. The same logic works no matter what `T` is.  
    // 2. Callers choose the real type at compile time.  
    // 3. Rust generates a specialized copy of the code for each concrete type used.
    // You reach for `T` (or any other generic parameter name) whenever you want **reuse without runtime cost**:
    // * Containers that hold any value (`Vec<T>`, `Option<T>`, `Box<T>`).  
    // * Algorithms that operate on any numeric type (`fn max<T: Ord>(a: T, b: T) -> T`).  
    // * Wrapper types that add behaviour (`struct TreeNode<T> { value: T, … }`).  
    // * Traits that describe capabilities independent of the concrete type (`impl<T> Clone for MyType<T>`).
    // In short, use `T` when you need **type-safe, zero-cost abstractions** that work for *any* type the caller supplies.

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