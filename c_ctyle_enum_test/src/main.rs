enum CLikeEnum {
    ERR_OK = 0,
    ERR_INVAL,
    ERR_MOTOR = 10,
    ERR_NOT,
}

fn main() {
    let err = CLikeEnum::ERR_OK;
    println!("{}", err as i32);
    println!("{}", CLikeEnum::ERR_INVAL as i32);
    println!("{}", CLikeEnum::ERR_MOTOR as i32);
    println!("{}", CLikeEnum::ERR_NOT as i32);
}
