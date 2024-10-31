macro_rules! create_function {
    ($function_name:ident) => {
        fn $function_name() {
            println!("you called {}()", stringify!($function_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

macro_rules! test {
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} and {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    };
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} or {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

macro_rules! find_min {
    ($x:expr) => ($x);
    ($x:expr, $($rest:expr),+ ) => (
        std::cmp::min($x, find_min!($($rest),+))
    );
}

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    println!("Hello, world!");
    foo();
    bar();

    print_result!(1u32 + 1);
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });

    test!(true; or false);
    test!(1 + 1 == 2; and 2 + 2 == 5);

    print_result!(find_min!(1, 2, 3, 4));

    calculate! {
        eval 1 + 2
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
