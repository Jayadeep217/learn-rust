fn main() {
    println!("====temp.rs====");

    // for i in 0..=1000 {
    //     print!("|{}|", i);
    // }

    moves();
}

fn moves() {
    let s1 = String::from("s1 string");
    let s2 = String::from("s2 string");
    /*
    error 
    let mvs1 = s1;
    let mvs2 = s2;
    let refs1 = &s1;
    let refs2 = &s2;
    */

    let mvs1 = s1;
    let mvs2 = s2;

    // println!("s1 : {}\ns2 : {}", s1, s2);
    println!("mvs1 : {}\nmvs2 : {}", mvs1, mvs2);
    // println!("refs1 : {}\nrefs2 : {}", refs1, refs2);
}
