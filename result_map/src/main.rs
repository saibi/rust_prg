use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let twenty = multiply("10", "2");
    print(twenty);

    let err = multiply("err", "2");
    print(err);

    early_return();
    intruducing_question_mark();
}

fn early_return() {
    fn mul(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(first_number) => first_number,
            Err(e) => return Err(e),
        };

        let second_number = match second_number_str.parse::<i32>() {
            Ok(second_number) => second_number,
            Err(e) => return Err(e),
        };

        Ok(first_number * second_number)
    }

    fn pr(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    // main
    println!("early_return");
    let twenty = mul("10", "2");
    pr(twenty);
    let err = mul("err", "2");
    pr(err);
}

fn intruducing_question_mark() {
    fn mul(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;

        Ok(first_number * second_number)
    }

    fn pr(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }

        // old try!
        // let val = try!(result);
        // println!("n is {}", val);
    }

    // main
    println!("intruducing_question_mark");
    let twenty = mul("10", "2");
    pr(twenty);
    let err = mul("err", "2");
    pr(err);
}
