#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}
impl<T> std::ops::Add for Pair<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            first: self.first + rhs.first,
            second: self.second + rhs.second,
        }
    }
}

fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    let left_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair: Pair<i32> = Pair {
        first: 10,
        second: 5,
    };

    let result = add(left_pair, right_pair);
    println!("result: {:?}", result);

    println!("Sum: {}", add(3.114, 2.71));

    let left_pair: Pair<i32> = Pair {
        first: 5,
        second: 10,
    };

    let right_pair: Pair<i32> = Pair {
        first: 10,
        second: 5,
    };
    println!("Sum: {:?}", left_pair + right_pair);
}
