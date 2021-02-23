#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fav_color: Color,
}

#[derive(Debug)]
enum Color {
    Red (String),
    Blue,
    Green,
}

impl Person {
    pub fn print(self) -> String {
        format!("name = {}, age = {} has {} children", 
            self.name, self.age, self.children)
    }
}

fn main() {
    let p = Person{
        name: "Reza".to_string(),
        age:32,
        children:2,
        fav_color:Color::Green,
    };

    let c = Color::Red("Hello".to_string());

    match c {
        Color::Red(s) => println!("It is Red! {}", s),
        Color::Blue => println!("It is Blue!"),
        Color::Green => println!("It is green!"),
    }
    
    println!("Hello, {:?}!", p);
}
