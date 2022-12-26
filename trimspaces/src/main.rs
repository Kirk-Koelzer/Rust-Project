fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Tests passed!");
}

fn trim_spaces(tests: &str) -> &str {
    let mut beg = 0;
    let mut end = 0;
    println!("{}", tests);
    for (index, item) in tests.chars().enumerate() {
        if item != ' ' {
            beg = index;
            break;
        }
    }

    for (index, item) in tests.chars().rev().enumerate() {
        if item != ' ' {
            end = tests.len() - index;
            break;
        }
    }
    &tests[beg..end]
}
