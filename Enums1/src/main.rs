enum TrafficLight {
    Red,
    Yellow,
    Green,
}

enum Notification {
    Email { from: String, to: String },
    SMS { phoneno: String, msg: String },
}

impl TrafficLight {
    fn call(&self) {
        match self {
            TrafficLight::Red => {
                println!("Stop");
            }
            TrafficLight::Yellow => {
                println!("Slow");
            }
            TrafficLight::Green => {
                println!("Go");
            }
        }
    }
}

impl Notification {
    fn call(&self) {
        match self {
            Notification::Email { from, to } => {
                println!("sending email from :: {}, to :: {}", from, to)
            }
            Notification::SMS { phoneno, msg } => {
                println!("sending msg :: {}, to :: {}", msg, phoneno)
            }
        }
    }
}

fn main() {
    println!("Try Enums in Rust");
    let lights1 = TrafficLight::Red;
    let lights2 = TrafficLight::Yellow;
    let lights3 = TrafficLight::Green;
    let notify1 = Notification::Email {
        from: String::from("abc@gmail.com"),
        to: String::from("xyz@gmail.com"),
    };
    let notify2 = Notification::SMS {
        phoneno: String::from("+1234567890"),
        msg: String::from("Hello Enums in Rust"),
    };

    lights1.call();
    lights2.call();
    lights3.call();
    notify1.call();
    notify2.call();

    // Create different shape variants
    let circle = Shape::Circle(5.6);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.1, 6.2);
    println!("Circle area: {}", cal_shape(circle));
    println!("Square area: {}", cal_shape(square));
    println!("Rectangle area: {}", cal_shape(rectangle));
}

/// Shape enum - represents different geometric shapes
/// Each variant can hold different data (tuple-like)
/// 
/// # Variants
/// * `Circle(f64)` - Circle with radius
/// * `Square(f64)` - Square with side length
/// * `Rectangle(f64, f64)` - Rectangle with length and breadth
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

/// Calculates area based on shape type
/// Uses pattern matching to handle each variant
/// 
/// # Arguments
/// * `shape` - Shape enum instance (ownership transferred)
/// 
/// # Returns
/// Area as f64
fn cal_shape(shape: Shape) -> f64 {
    // match expression - like switch but more powerful
    // Must handle ALL variants (exhaustive)
    let ans = match shape {
        // Destructure Circle variant and extract radius
        Shape::Circle(radius) => 3.14 * radius * radius,
        // Destructure Square variant and extract side
        Shape::Square(side) => side * side,
        // Destructure Rectangle variant and extract both dimensions
        Shape::Rectangle(length, breadth) => {
            length * breadth
        },
    };
    return ans;
}