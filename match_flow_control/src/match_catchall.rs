fn add_fancy_hat() {
    println!("Adding fancy hat!");
}
fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces!", num_spaces);
}

fn main() {
    let dice_roll = 7;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other_val => move_player(other_val),
    }
}
