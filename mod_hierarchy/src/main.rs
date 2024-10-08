mod my;

fn function() {
    println!("called `function()`");
}
fn main() {
    function();
    my::function();
    my::indirect_access();
    // my::private_function();

    my::nested::function();
}
