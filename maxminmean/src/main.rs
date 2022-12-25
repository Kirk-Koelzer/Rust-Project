use std::array;

fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    // let mut min: i32;
    // let mut mean: f64;
   
    let mut new_max: i32 = 0;  
    let mut new_min: i32 = 0;  

    for item in numbers.iter() {
        // println!("Current number is {}", item);
        if  item > &new_max {
            // println!(" current max number {} is not bigger than {}",new_max, item,);
            new_max = *item;
        }else {
            // println!(" current number {} is bigger than {}",item, new_max);
        }
    }
    

    for item in numbers.iter() {
        // println!("Current number is {}", item);
        if  item < &new_min {
            // println!(" current max number {} is not bigger than {}",new_min, item,);
            new_min = *item;
        }else {
            // println!(" current number {} is bigger than {}",item, new_min);
        }
    }
println!("{}", new_min);
println!("{}", new_max);


    // calc_mean();
    // calc_min();
    

    // assert_eq!(max, 56);
    // assert_eq!(min, -18);
    // assert_eq!(mean, 12.5);
    // println!("Tests passed!");
}



fn calc_min(){
    
}

fn calc_mean(){
    
}