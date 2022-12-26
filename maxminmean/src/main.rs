fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;
    let mut i: f64;

    max = 0;
    min = 0;
    i = 0.0;

    for &item in numbers.iter() {
        // println!("Current number is {}", item);
        if item > max {
            // println!(" current max number {} is not bigger than {}",new_max, item,);
            max = item;
        } else {
            // println!(" current number {} is bigger than {}",item, new_max);
        }
    }

    for &item in numbers.iter() {
        // println!("Current number is {}", items);
        if item < min {
            // println!(" current max number {} is not bigger than {}",new_min, item,);
            min = item;
        } else {
            // println!(" current number {} is bigger than {}",item, new_min);
        }
    }

    for &item in numbers.iter() {
        // println!("Current number  from array is {}", item);
        i += item as f64;
        // println!("Current total is {}", meantotal);
    }
    mean = i / numbers.len() as f64;


    println!("{}", min);
    println!("{}", max);
    println!("{}", mean);
    

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}

