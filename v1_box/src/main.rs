#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    /*
    All values in Rust are stack allocated by default. 
    Values can be boxed (allocated on the heap) by creating a Box<T>. 
    A box is a smart pointer to a heap allocated value of type T. 
    When a box goes out of scope, its destructor is called, 
    the inner object is destroyed, and the memory on the heap is freed.
    */
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut list = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut v) = list.next {
        v.add_up(10);
    }

    let mut v: Vec<String> = Vec::with_capacity(4);

    v.push("hello".to_string());
    v.push("goodbye".to_string());

    println!("v.len = {}, capacity = {}", v.len(), v.capacity());

    println!("Our list is equal: {:?}", list);

    let s = " hello! ".to_string();
    let p = s.trim();
    println!("p = '{}', and s string exist = '{}'", p, s);
    let fstr = "help me find home";

    println!("{}", string_find_f(fstr));
}

fn string_find_f(s: &str) -> &str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}
