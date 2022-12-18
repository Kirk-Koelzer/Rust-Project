use core::num;

fn main() {

//PRIMITIVE DATA TYPES

    let mut x:f64 = 255.05;
    x = x + 1.0;
    println!("Hello, world!");
    println!("x is {}", x);

    x = 20.2;
    println!("x is {}", x);


    let a = 10;
    let b = 2.2;
    let c = a as f64 + b;

    println!("c is equal to {:.3}", c);
    println!("c is equal to {:08.3}", c);
    println!("c is equal to {0:08.3}\n and a is {1} and once again c is equal to {0}", c, a);

    ////bitwise ops
    let mut value = 0b1111_0101u8;
    println!("value is {0}.", value);
    println!("value is {:08b}.", value);

    //////Flipping the bits
    value = !value;
    println!("value is {:08b}.", value);

    value = value & 0b1111_0111;
    println!("value is {:08b}.", value);

    println!("bit 6 is {}.", value & 0b0100_0000);
    
    value = value | 0b0100_0000;
    println!("value is {:08b}.", value);

    value = value ^ 0b0101_0101;
    println!("value is {:08b}.", value);

    let letter = 'a';
    let number = '1';

    println!("the letter is {} and the number is {}", letter, number);

    let finger = '\u{261D}';

    println!("the unicode char is {}", finger);
}
