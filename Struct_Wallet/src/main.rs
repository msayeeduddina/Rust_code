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
}
