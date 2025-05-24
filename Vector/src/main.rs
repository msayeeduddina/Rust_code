fn main() {
    // Create an empty, growable vector of i32 integers using explicit type annotation
    let mut a: Vec<i32> = Vec::new();
    // Add elements to the vector using push
    a.push(1);
    a.push(2);
    a.push(3);
    // Print the vector using debug formatting
    println!("{:?}", a);

    // Create another empty vector of i32 using turbofish syntax for type annotation
    let mut b = Vec::<i32>::new();
    b.push(4);
    b.push(5);
    b.push(6);
    // Remove the last element using pop (returns Option<T>)
    b.pop();
    println!("{:?}", b);

    // Create a vector with initial values using the vec! macro
    let mut c = vec![7, 8, 9];
    c.push(10);
    println!("{:?}", c);
}
