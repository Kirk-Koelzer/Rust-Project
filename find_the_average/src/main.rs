fn main() {
    let a = 13;
    let b = 2.3;
    let c = 120.0;
    let d  = 3;
    let average = (a as f64 + b + c) / d as f64; 

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
