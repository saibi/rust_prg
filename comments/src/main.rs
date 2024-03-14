fn main() {
    // In general, the {} will be automatically replaced with any arguments. These will be stringified.

    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside {} determines which additional argument will be replaced.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "bob");

    // Named arguments can also be used.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick bown fox",
        verb = "jumps over"
    );

    // different formatting can be invoked by specifying the format character after a ':'
    println!("base 10: {}", 69420);
    println!("base 2: {:b}", 69420);
    println!("base 8: {:o}", 69420);
    println!("base 16: {:x}", 69420);
    println!("base 16: {:X}", 69420);

    // right-align text with a specified width
    println!("{number:>width$}", number = 1, width = 6);

    // pad numbers with extra zeroes
    println!("{number:>0width$}", number = 1, width = 6);

    // left-align text with a specified width
    println!("{number:<width$}|", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    // This will not compile because `Structure` does not implement fmt::Display.
    println!("This struct `{}` won't print...", Structure(3));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);
    // Pretty print
    println!("{:#?}", peter);

    let complex = Complex(3.3, 7.2);
    println!("complex = {}", complex);

    let v = List(vec![1, 2, 3, 4, 5]);
    println!("{}", v);
}

#[allow(dead_code)] // disable `dead_code` which warn against unused module
struct Structure(i32);

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Structure({})", self.0)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug)]
struct Complex(f64, f64);

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "real: {} + img: {}i", self.0, self.1)
    }
}

struct List(Vec<i32>);

impl std::fmt::Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        write!(f, "]")
    }
}
