fn main() {
    let text = "apple,,banana,,,cherry";

    let v = text
        .split(",")
        .filter(|&x| !x.is_empty())
        .collect::<Vec<&str>>();

    println!("vector of String = {:?}", v);
    for &e in &v {
        println!("{:}", e);
    }
}
