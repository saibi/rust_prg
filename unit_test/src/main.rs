fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn bad_add(a: i32, b: i32) -> i32 {
    a + b + 1
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("division by zero");
    } else if a < b {
        panic!("division of {} by {} would result in a non-integer", a, b);
    } else {
        a / b
    }
}

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", add(1, 2));
    println!("1 + 2 = {}", bad_add(1, 2));
    println!("sqrt(4.0) = {:?}", sqrt(4.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 10);
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 0);
    }

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    #[ignore]
    fn test_bad_add() {
        assert_eq!(bad_add(1, 2), 3);
    }
}
