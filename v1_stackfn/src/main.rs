/*
Things a function call need
(1) where to go after the function is finished
(2) a pointer
(3) space for its parameters
(4) space for its variables

*/

fn main() {
    println!("fact(4) is {}", factorial(4));
    println!("tail recursion fact(4) is {}", factorial_tail(4, 1));
}

/*
fac 4
4 * fac (3)
4 * ( 3 * fac (2))
4 * ( 3 * (2 * fac(1)))
4 * ( 3 * (2 * 1))
...
*/
fn factorial(n:i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    n * factorial(n-1)
}

/*
fac 4, 1
fac 3, 4 =>  (4 - 1) (1 * 4)
fac 2, 12 => (3 - 1) (3 * 4)
fac 1, 24 => (1 * 24)
*/
fn factorial_tail(n:i32, r:i32) -> i32 {
    if n <= 1 {
        return r;
    }

    factorial_tail(n-1, n*r)
}
