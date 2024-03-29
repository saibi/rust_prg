fn binding_test() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
}

fn mutability_test() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
}

fn scope_test() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);
}

fn declare_first() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    // let another_binding;
    // println!("another binding: {}", another_binding);
    // another_binding = 1;
}
fn main() {
    binding_test();
    mutability_test();
    scope_test();
    declare_first();
}
