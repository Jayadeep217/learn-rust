fn main() {
    //add
    let add = 5.0 + 1.10;
    println!("{}", add);

    //sub
    let diff = 12 - 98;
    println!("{}", diff);

    //mull
    let mul: u128 = 12342124 * 1232134124124;
    println!("{}", mul);

    //div
    let div = 56 / 2;
    let div_1 = 5.6 / 2.5;
    println!("{}\n{}", div, div_1);

    let truncated = -5 / 3;
    println!("{}", truncated);

    //remainder
    let rem = -5 % 3;
    println!("{}", rem);

    //functions
    fn test_function_inside() {
        println!("test func inside main! {}", return_function());
    }

    test_function_inside();
    test_function_outside();
}

//functions
fn test_function_outside() {
    println!("test func outside main! {}", five());
}

fn five() -> i8{
    5
}

fn return_function() -> isize {
    let mut a:isize = five().into();
    a = 12343414 + a;
    return  a.into();
}