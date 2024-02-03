fn main() {
    let hello = String::from("안녕하세요");
    println!("{}", hello.len());
    println!("{}", hello.chars().count());

    let s = &hello[0..3];
    println!("{}", s);

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{b}");
    }

    // let s1 = String::from("hello");
    // let h = s1[0];

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s = s1 + "-" + &s2 + "-" + &s3;
    // let s = format!("{}-{}-{}", s1, s2, s3);
    // println!("s: {}", s);

    // let mut s = String::new();
    // let data = "initial contents";
    // let mut s2 = data.to_string();
    // let mut s3 = "initial contents".to_string();
    // let mut s4 = String::from("안녕하세요");

    // s.push_str("hello");
    // s4.push_str("안녕하세요");
    // s2.push('A');
    // s3.push_str(&s2);

    // println!("s: {}", s);
    // println!("s2: {}", s2);
    // println!("s3: {}", s3);
    // println!("s4: {}", s4);

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let mut s = String::from("lo");
    // s.push('l');

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("s3: {}", s3);
}
