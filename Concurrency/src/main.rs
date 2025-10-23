use std::thread; // bring module `thread` into scope
use std::time::Duration; // bring `Duration` into scope
use std::sync::{
    Arc,
    Mutex
}; // bring two concurrent types into scope

// ---------- 1.  Background thread that may be killed early ----------
fn check_thread_incomplete() { // `fn` declares a function
    thread::spawn( || { // `spawn` starts OS thread, `||` is zero-arg closure
        for i in 1..25 { // `for` loop over range `1..25` (exclusive)
            println!("number {} from the spawned thread!", i); // `println!` macro prints
            thread::sleep(Duration::from_millis(1)); // sleep current thread 1 ms
        }
    }); // spawned thread is now running

    for i in 1..20 { // main thread continues concurrently
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
} // end of function = main thread ends,
// child thread is forcibly killed here

// ---------- 2.  Background thread that runs to completion ----------
fn check_thread_complete() {
    let handle = thread::spawn( || { // keep `JoinHandle` in `handle`
        for i in 1..25 {
            println!("number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 { // main thread work
        println!("number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // block until child finishes; `unwrap` panics on panic
}

// ---------- 3.  Shared state with atomic reference counting & mutex ----------
fn check_bank_feature() {
    struct Bank { // `struct` defines a new named type
        balance: f32, // field `balance` of type `f32`
    }

    fn withdraw(bank: &Arc<Mutex<Bank>> , amt: f32) { // `&` = borrow, `Arc` = shared ownership, `Mutex` = mutual exclusion
        let mut account = bank.lock().unwrap(); // `lock()` blocks until acquired; `unwrap` panics if poisoned
        if account.balance < 5.00 { // compare field
            println!("Current balance {} is too low", account.balance);
        } else {
            account.balance -= amt; // mutate inside mutex
            println!("Withdrew {}; new balance {}", amt, account.balance);
        }
    } // `account` guard dropped here → lock released

    fn customer(bank: &Arc<Mutex<Bank>> ) { // wrapper chosen for `spawn` closure
        withdraw(bank, 5.0); // call `withdraw` with 5.0
    }

    let bank = Arc::new(Mutex::new(Bank {
        balance: 20.0
    })); // `Arc` allows shared ownership across threads

    let handles: Vec<_> = (0..10) // range `0..10` produces 0-9
        .map( | _ | { // `map` turns each index into a handle
            let bank_clone = Arc::clone( & bank); // `clone` increments ref-count (cheap)
            thread::spawn(move || customer( & bank_clone)) // `move` closure takes ownership of `bank_clone`
        })
        .collect(); // collect iterators into `Vec<JoinHandle<_>>`

    for h in handles { // iterate over handles
        h.join().unwrap(); // wait for each thread to finish
    }

    println!("Final balance: {}", bank.lock().unwrap().balance); // lock again to read final value
}

fn main() { // program entry point
    check_thread_incomplete();
    println!("--------------------------------");
    check_thread_complete();
    println!("--------------------------------");
    check_bank_feature();
}