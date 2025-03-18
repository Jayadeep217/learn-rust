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
    for ele in &arr {
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

    array_iteration();
}

fn array_iteration() {
    let arr: [i32; 8] = [5, 7, 9, 4, 2, 45, 7, 2];
    //=====================================================================
    println!(
        r#"▶️   simple for loop.
✅ Consumes the array (moves ownership).
✅ Works only for arrays, not vectors (use .iter() for vectors)."#
    );
    for item in arr {
        println!("{}", item);
    }
    //=====================================================================
    println!(
        r#"▶️  simple for loop Iterate by Reference (&).
✅ Safer than moving elements.
✅ Useful when elements shouldn't be consumed."#
    );
    for item in &arr {
        println!("{}", item);
    }
    //=====================================================================
    println!(
        r#"▶️  Using .iter() (Explicit Iteration)
✅ arr.iter() returns an iterator over references (&T).
✅ Elements are not moved, keeping arr accessible."#
    );
    for item in arr.iter() {
        println!("{}", item);
    }
    //=====================================================================
    println!(
        r#"▶️  Using .iter_mut() (Mutable Iteration)
✅ Works for mutable arrays.
✅ *item += 1; dereferences the mutable reference."#
    );
    let mut arr_1 = [10, 20, 30, 40];
    for item in arr_1.iter_mut() {
        *item += 1;
    }
    println!("{:?}", arr);
    //=====================================================================
    println!(
        r#"▶️  Using .enumerate() (Index + Value)
✅ Preserves original array.
✅ Efficient & readable."#
    );
    for (index, value) in arr.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
    //=====================================================================
    println!(
        r#"▶️  Using while let Loop (Destructuring)
✅ Works for iterators that may return None (like consuming a vector).
✅ Explicit iteration via .next()."#
    );
    let mut arr_2 = [10, 20, 30, 40].iter();
    while let Some(value) = arr_2.next() {
        println!("{}", value);
    }
    //=====================================================================
    println!(
        r#"7️⃣  Using .into_iter() (Consumes Ownership)
✅ Consumes arr completely (ownership moved).
❌ arr cannot be used after this."#
    );
    for item in arr.into_iter() {
        println!("{}", item);
    }
    //=====================================================================
    println!(
        r#"8️⃣  Using Iterators with .map()
✅ Functional approach.
✅ Returns a new collection (Vec<i32>)."#
    );
    let squared: Vec<i32> = arr.iter().map(|x| x * x).collect();
    println!("{:?}", squared);
    //=====================================================================
    println!(
        r#"9️⃣  Using for_each() (Functional Iteration)
✅ Clean syntax but less flexible than for loop.
✅ Cannot break early (unlike for loops)."#
    );
    arr.iter().for_each(|&x| println!("{}", x));
}
