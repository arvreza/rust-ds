#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    /*
        if you pass self or mut self then it means I want to take the hold of the 
        object and I will not give it back. Unless you want to return it.
        the most common use is to work with the reference of the object. In Rust it is called borrowed.
    */
    pub fn print(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n;
    }

    //pretty useful to kill the object through method signature
    pub fn dropme(self) {}
}

fn main() {
    let mut p = Person::new("Reza".to_string(), 50);
    let s = p.print();

    p.age_up(10);

    println!("{}", s);
    println!("{:?}", p);
}
