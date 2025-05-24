fn main() {
    let x: i32 = 5;
    let x: i32 = x - 2;
    println!("Value of X is = {x}");
    {
        let x: i32 = x * 2;
        println!("Value of X is = {x}");
    }
    println!("Value of X is = {x}");
    let a = [1, 2, 3, 4, 5];
    let zeroIdx = a[0];
    let oneIdx = a[1];
    println!("Value of 0 is = {zeroIdx}");
    println!("Value of 1 is = {oneIdx}");
}