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
}
