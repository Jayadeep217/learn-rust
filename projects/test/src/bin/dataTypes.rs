use std::any::type_name;

fn get_type<T>(_: &T) -> &str {
    type_name::<T>()
}

fn main() {
    // integer
    let int: isize = 867964312546786562;
    let int_d = 2147483647;
    println!(
        "type {} : {} || type {} : {}",
        get_type(&int),
        int,
        get_type(&int_d),
        int_d
    );

    // float
    let flo32: f32 = 12.4232335;
    let flo64: f64 = 12.12345678901234567890;
    let flo32_d = 12.4232335;
    let flo64_d = 12.12345678901234567890;
    println!(
        "type {} : {} || type {} : {} || type {} : {} || type {} : {}",
        get_type(&flo32),
        flo32,
        get_type(&flo64),
        flo64,
        get_type(&flo32_d),
        flo32_d,
        get_type(&flo64_d),
        flo64_d
    );

    // bool
    let boo: bool = true;
    println!("type {} : {}", get_type(&boo), boo);

    //char
    let c: char = 'థ';
    println!("type {} : {}", get_type(&c), c);

    //Array
    let arr: [i8; 5] = [2, 3, 3, 5, 56];
    for ele in arr {
        println!("{}", ele);
    }
    println!("type {} : {:?}", get_type(&arr), arr);

    //tuple
    let tup: (i8, [i8; 3], (f64, &str, bool)) = (2, [4, 5, 6], (21.54, "asd", false));
    println!("{:?}", tup);

    //string
    let s1 = "hello!";
    let s2: &str = "asdsadasfsa";
    let s3 = String::from("Hello, Rust!");
    println!(
        "type {} : {} || type {} : {} || type {} : {}",
        get_type(&s1),
        s1,
        get_type(&s2),
        s2,
        get_type(&s3),
        s3
    );
}
