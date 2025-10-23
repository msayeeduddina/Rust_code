mod restaurant; // Bring the `restaurant` module into scope of this crate.
use crate::restaurant::order_food; // Bring the `order_food` function into scope so we can call it without the full path.

fn main() {
    order_food(); // Call the `order_food` function from the `restaurant` module.
}