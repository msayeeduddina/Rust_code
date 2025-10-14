// `struct` keyword is used to define a structure, similar to classes in other languages.
struct Wallet {
    name: String, // `String` is a growable, UTF-8 encoded string type.
    balance: u32, // `u32` is an unsigned 32-bit integer type.
}

// `impl` keyword is used to implement methods for a struct.
impl Wallet {
    // `&self` is a reference to the instance of the `Wallet` struct.
    fn show_bal(&self) {
        // `println!` is a macro for printing formatted text to the console.
        println!("Name: {}, have Balance: {}.", self.name, self.balance);
    }

    // `&mut self` is a mutable reference to the instance of the `Wallet` struct,
    // allowing the method to modify the struct's fields.
    fn do_expense(&mut self, expenditure: u32) {
        if expenditure <= self.balance {
            self.balance -= expenditure;
            println!("Name: {}, have Balance: {}.", self.name, self.balance);
        } else {
            println!("Insufficient Balance");
        }
    }
}

// `fn main()` is the entry point of the program.
fn main() {
    // `let mut` declares a mutable variable.
    let mut wallet = Wallet {
        // `String::to_string()` converts a string literal to a `String`.
        name: "DrChain".to_string(),
        balance: 200,
    };

    // Calling methods on the `wallet` instance using dot notation.
    wallet.show_bal();
    wallet.do_expense(50);
    wallet.show_bal();

    user_details();
    cal_area();
}

fn user_details() {
    // Create an instance of the User struct
    // All fields must be initialized
    let user_data = User {
        active: true,
        username: String::from("Hello World"),
        email: String::from("Hello@World.com"),
        sign_in_count: 123,
    };
    // Access struct fields using dot notation
    println!("User name is {}, email is {}, sign_in_count is {}, and user is active: {}", user_data.username, user_data.email, user_data.sign_in_count, user_data.active);
}

/// User struct - custom data type grouping related data
/// Structs are like classes in other languages (but without methods here)
/// 
/// # Fields
/// * `active` - Whether the user account is active
/// * `username` - User's display name
/// * `email` - User's email address
/// * `sign_in_count` - Number of times user has signed in
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Rectangle struct - stores dimensions
/// 
/// # Fields
/// * `width` - Width of the rectangle in pixels
/// * `height` - Height of the rectangle in pixels
struct Rect {
    width: u32,
    height: u32,
}

/// Implementation block for Rect
/// Contains methods (functions) associated with the Rect struct
impl Rect {
    /// Calculates the area of the rectangle
    /// 
    /// # Arguments
    /// * `&self` - Immutable borrow of the instance (read-only access)
    /// 
    /// # Returns
    /// Area as u32 (width * height)
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter( &self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

fn cal_area() {
    // Create an instance of Rect
    let rect: Rect = Rect { 
        width: 30, 
        height: 50 
    };
    // Call method using dot notation
    // rect.area() is syntactic sugar for Rect::area(&rect)
    println!("Area of the rectangle is {} square pixels.", rect.area());
    println!("Perimeter of the rectangle is {} pixels.", rect.perimeter());
}