use std::array;

fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean: f64 = 0.0;
    let mut meantotal: f64 = 0.0;

    println!("{}", min);
    println!("{}", max);
    println!("{}", mean);
}

fn calc_max() {
    for item in numbers.iter() {
        // println!("Current number is {}", item);
        if item > &max {
            // println!(" current max number {} is not bigger than {}",new_max, item,);
            max = *item;
        } else {
            // println!(" current number {} is bigger than {}",item, new_max);
        }
    }
}

fn calc_min() {
    for item in numbers.iter() {
        // println!("Current number is {}", item);
        if item < &min {
            // println!(" current max number {} is not bigger than {}",new_min, item,);
            min = *item;
        } else {
            // println!(" current number {} is bigger than {}",item, new_min);
        }
    }
}

fn calc_mean() {
    for item in numbers.iter() {
        // println!("Current number  from array is {}", item);
        meantotal = meantotal + *item as f64;
        // println!("Current total is {}", meantotal);
    }

    mean = meantotal / numbers.len() as f64;
}
