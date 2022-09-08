fn main() {
    let name = String::from("Bird");
    let bird = Bird { name, attack: 5 };
    bird.print_name();
}

struct Bird {
    name: String,
    attack: u64,
}

impl Bird {
    fn print_name(&self) {
        println!("{}", self.name);
    }
}
