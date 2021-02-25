pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {

    let mut st = Stepper {
        curr:2, 
        step:6, 
        max:44
    };

    loop {
        match st.next() {
            Some(v) => println!("Loop {}", v),
            None => break,
        }
    }

    let mut st = Stepper {
        curr:2, 
        step:6, 
        max:44
    };

    while let Some(n) = st.next() {
        println!("While {}", n);
    }

    let it = Stepper {
        curr:5, 
        step:77, 
        max:440
    };

    for i in it {
        println!("for loop {}", i)
    }

    // here 0..10 is just a syntax sugar. It is basically a range of objects. 
    // for i in 0..10 {
    //     println!("for loop {}", i)
    // }
    println!("All done!");
}
