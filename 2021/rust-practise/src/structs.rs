pub fn run() {
    basicstruct();
    tuplestruct();
    fnstruct();
}

fn basicstruct() {
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let mut c = Color{ red: 255, green: 0, blue: 128 };

    c.green = 64;
    println!("{} {} {}", c.red, c.green, c.blue)
}

fn tuplestruct() {
    struct Color(u8, u8, u8);

    let mut c = Color(0, 128, 255);
    
    c.0 = 64;
    println!("{} {} {}", c.0, c.1, c.2)
}

fn fnstruct() {
    struct Person {
        first: String,
        last: String
    }

    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person {
                first: first.to_string(),
                last: last.to_string()
            }
        }
        fn name(&self) -> String {
            format!("{} {}", self.first, self.last)
        }

        fn set_last(&mut self, last: &str) {
            self.last = last.to_string();
        }
        fn to_tuple(self) -> (String, String) {
            (self.first, self.last)
        }
    }

    let mut p = Person::new("Ben", "bennn"); // class fns = :: (e.g. no &self)

    println!("{}", p.name()); // method fns = . (e.g. with &self);

    p.set_last("nay"); //this is why p needs to be `mut`
    println!("{}", p.name());
    println!("{:?}", p.to_tuple());
}
