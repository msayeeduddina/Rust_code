fn main() {
    let employee_data: (&str, u8) = ("John", 30);
    println!("Employee Name is : {}", employee_data.0);
    println!("Employee Age is : {}", employee_data.1);
    let (emp_name, emp_age) = employee_data;
    println!("Employee Name is : {}", emp_name);
    println!("Employee Age is : {}", emp_age);
}
