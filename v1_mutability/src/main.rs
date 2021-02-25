#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut p = Person {
        name: "Reza".to_string(),
        age: 50,
    };

    // the following statement will not work because  Person does not implement copy
    //let p2 = p;
    let p2 = p.clone();

    //now we can change the name in p
    p.name.push_str(" Arbabi");

    println!("p = {:?}, p2 = {:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let pnt2 = pnt;

    pnt.x += 5;
    println!("pnt = {:?}, pnt2 = {:?}", pnt, pnt2);

}
