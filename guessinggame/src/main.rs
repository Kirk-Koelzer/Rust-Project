use rand::thread_rng;
use rand::Rng;
use std::io;

fn main() {
    let answer = thread_rng().gen_range(1..10);
    // println!("the random number is {}", answer);
    let mut numberinput = get_input();
    println!("the after function is {}", numberinput);
    loop {
        if numberinput < answer {
            println!("Too Low");
            numberinput = get_input();
            continue;
        } else if numberinput > answer {
            println!("Too High");
            numberinput = get_input();
            continue;
        } else if numberinput == answer {
            println!("Congratz you have guessed the right answer!!!");
            // Exit this loop
            break;
        }
        println!("The answer is {} and you entered {}", answer, numberinput);
    }
}

fn get_input() -> i32 {
    let mut stringinput = String::new();
    println!("Enter your guess:");
    io::stdin().read_line(&mut stringinput);
    let numberinput: i32 = stringinput.trim().parse().unwrap();
    println!("you entered {}", numberinput);
    return numberinput;
}
