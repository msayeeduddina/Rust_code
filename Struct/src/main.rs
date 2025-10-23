// `struct` keyword is used to define a structure, similar to classes in other languages.
struct Wallet {
    name: String, // `String` is a growable, UTF-8 encoded string type.
    balance: u32, // `u32` is an unsigned 32-bit integer type.
}

// `impl` keyword is used to implement methods for a struct.
impl Wallet {
    // `&self` is a reference to the instance of the `Wallet` struct.
    fn show_bal( &self) {
        // `println!` is a macro for printing formatted text to the console.
        println!("Name: {}, have Balance: {}.", self.name, self.balance);
    }
    // `&mut self` is a mutable reference to the instance of the `Wallet` struct,
    // allowing the method to modify the struct's fields.
    fn do_expense( & mut self, expenditure: u32) {
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
    calc_size();
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
    fn area( & self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter( & self) -> u32 {
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

// `trait` keyword defines a shared interface that types can implement.
// Traits are similar to interfaces in other languages.
// This Shape trait defines behavior that all shapes must have.
trait Shape {
    // `fn new` is a constructor method signature in the trait.
    // `Self` (capital S) refers to the type implementing this trait.
    // All implementing types must provide a constructor with these parameters.
    fn new(length: f32, width: f32) -> Self;
    // `&self` borrows the instance immutably (read-only access).
    // `-> f32` specifies the return type as a 32-bit floating point number.
    // All shapes must implement an area calculation method.
    fn area( & self) -> f32;
}

// `struct` keyword defines a structure to represent a rectangle.
// Structs group related data together as a single type.
struct Rectangle {
    // `f32` is a 32-bit floating point type for decimal numbers.
    // Used for dimensions that can have fractional values.
    length: f32,
    width: f32,
}

// `struct` keyword defines a structure to represent a circle.
// Unlike Rectangle, Circle only needs one dimension (radius).
struct Circle {
    // `f32` stores the radius as a floating point number.
    radius: f32,
}

// `impl Trait for Type` syntax implements a trait for a specific type.
// This block provides the concrete implementation of Shape for Rectangle.
impl Shape for Rectangle {
    // Implements the `new` constructor method required by the Shape trait.
    // `fn new` is an associated function (called on the type, not an instance).
    // Takes two f32 parameters and returns a Rectangle instance.
    fn new(length: f32, width: f32) -> Rectangle {
        // Field init shorthand: when parameter names match field names.
        // This is equivalent to: Rectangle { length: length, width: width }
        // The last expression is automatically returned (no semicolon).
        Rectangle {
            length,
            width
        }
    }
    // Implements the `area` method required by the Shape trait.
    // `&self` borrows the Rectangle instance immutably.
    // Calculates area using the formula: length × width.
    fn area( & self) -> f32 {
        // `self.length` accesses the length field using dot notation.
        // `*` multiplies the two values together.
        // Last expression without semicolon is the return value.
        self.length * self.width
    }
}

// `impl Trait for Type` implements the Shape trait for Circle.
// This provides Circle-specific behavior for the Shape interface.
impl Shape for Circle {
    // Implements the `new` constructor for Circle.
    // Note: The trait requires two parameters, but Circle only needs radius.
    // `_width` has an underscore prefix to indicate it's intentionally unused.
    fn new(length: f32, _width: f32) -> Circle {
        // Only uses `length` parameter as the radius.
        // The `_width` parameter is ignored (design limitation of the trait).
        Circle {
            radius: length
        }
    }
    // Implements the `area` method for Circle.
    // Calculates area using the formula: π × radius².
    fn area( & self) -> f32 {
        // `self.radius * self.radius` calculates radius squared (r²).
        // `* 3.14` multiplies by an approximation of π (pi).
        // For better precision, use `std::f32::consts::PI` instead of 3.14.
        self.radius * self.radius * 3.14
    }
}

// `fn` keyword defines a function named calc_size.
// This function demonstrates creating shapes and calculating their areas.
fn calc_size() {
    // `let` keyword declares an immutable variable binding.
    // `Rectangle::new()` calls the associated function (constructor).
    // Type annotation is optional here (Rust infers it as Rectangle).
    let rec = Rectangle::new(10.0, 20.0);
    // Creates a Circle with radius 10.0.
    // Second parameter (0.0) is required by trait but ignored by Circle.
    let cir = Circle::new(10.0, 0.0);
    // `print!` macro prints formatted text without a newline.
    // `{}` is a placeholder that formats using the Display trait.
    // `.area()` method call uses dot notation to invoke the area method.
    println!("Area of Rectangle: {}", rec.area());
    // Prints the circle's area using the same pattern.
    // Note: Using `print!` instead of `println!` means no newline is added.
    println!("Area of Circle: {}", cir.area());
}