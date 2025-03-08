fn main() {
    println!("====temp.rs====");
    let arr: [i8; 5] = [2, 3, 3, 5, 56];
    for ele in arr {
        println!("{}", ele);
    }
    println!("{:?}", arr);
}
