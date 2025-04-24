#[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Implemented("smith".to_string())
    }
}

fn main() {
    let default_struct = Derived::default();
    println!("default_struct: {:#?}", default_struct);

    let almost_default_struct = Derived {
        y: "Y manual".into(),
        ..Derived::default()
    };
    println!("almost_default_struct: {:#?}", almost_default_struct);

    let nothing: Option<Derived> = None;
    println!("nothing: {:#?}", nothing.unwrap_or_default());
}
