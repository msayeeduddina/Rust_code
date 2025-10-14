/// Main entry point demonstrating boolean logic and control flow
fn main() {
    // Boolean variables
    let is_male = true;
    let is_above_18 = true;
    let has_id = false;
    let age = 25;
    
    // Basic if-else
    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }
    
    // Logical AND: && (both must be true)
    if is_male && is_above_18 {
        println!("You are a male and above 18");
    }
    
    // Logical OR: || (at least one must be true)
    if is_above_18 || has_id {
        println!("You are either above 18 or have an ID");
    }
    
    // Logical NOT: ! (inverts boolean)
    if !has_id {
        println!("You don't have an ID");
    }
    
    // else if chain for multiple conditions
    if age < 13 {
        println!("You are a child");
    } else if age < 18 {
        println!("You are a teenager");
    } else {
        println!("You are an adult");
    }
    
    // Comparison operators (==, !=, <, >, <=, >=) return bool
    if age >= 18 {
        println!("Eligible to vote");
    }
    
    // if as an expression - returns a value
    let status = if is_above_18 { "Adult" } else { "Minor" };
    println!("Status: {}", status);
    
    // Complex condition with parentheses
    if (is_male && is_above_18) || has_id {
        println!("Access granted");
    }
}