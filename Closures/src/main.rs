fn check_can_vote() {
    // Closure that tests voting eligibility: true if age ≥ 18
    let can_vote = |age: i32| -> bool { age >= 18 };
    println!("Can vote: {}", can_vote(8)); // prints false
}

fn check_sample_value() {
    // Variable captured by an immutable borrow
    let samp_val = 5;
    let get_samp = || println!("Sample value is: {}", samp_val);
    get_samp(); // execute the closure
}

fn check_use_function() {
    // Higher-order function: apply `func` to `a` and `b`, then return the result
    fn use_function<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32,
    {
        func(a, b) // return the computed value
    }

    let sum  = |a, b| a + b; // closure for addition
    let prod = |a, b| a * b; // closure for multiplication

    println!("5 + 4 = {}", use_function(5, 4, sum));
    println!("5 * 4 = {}", use_function(5, 4, prod));
}

fn main() {
    check_can_vote();      // demo eligibility check
    check_sample_value();  // demo simple capture & print
    check_use_function();  // demo passing closures to functions
}