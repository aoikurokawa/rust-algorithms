// #[derive(Debug)]
// pub enum Res<T, E> {
//     Thing(T),
//     Error(E),
// }

pub enum Option<T> {
    Some(T),
    None,
}

fn handler() {
    let a = divide(10, 5);

    let b = divide(10, 0);

    // match b {
    //     Res::Thing(v) => println!("val = {:?}", v),
    //     Res::Error(e) => println!("{:?}", e),
    // }
    if let Ok(v) = a {
        println!("val = {}", v);
    }

    // println!("a = {:?} b = {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot Divide by zero".to_string());
    }
    Ok(a / b)
}
