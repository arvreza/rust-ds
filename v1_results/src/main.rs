// pub enum Res<T, E> {
//     Thing(T),
//     Error(E),
// }

// in a sense rust does not have error handling like try catch and everything should be handled by programmers
// Result enum is a powerful tool to help you achieve that. 

fn main() {
    let a = divide(4, 5);
    let b = divide(10, 0);
    println!("a = {:?}, b={:?}", a, b);
}

fn divide (a:i32, b:i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err("Cannot divide by zero!".to_string());
    }

    return Result::Ok(a / b);
}