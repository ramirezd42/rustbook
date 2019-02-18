const PHRASES: [&str; 11] = [
    "Two turtle doves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings, badam-pam-pam",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a milking",
    "Nine ladies dancing",
    "Ten lords a leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    println!("On the first day of Christmas");
    println!("My true love gave to me");
    println!("A partridge in a pear tree\n");

    for i in 1..PHRASES.len() + 1 {
        println!("On the first day of Christmas");
        println!("My true love gave to me");
        print_phrases(i);
        println!("And a partridge in a pear tree\n")
    }
}

fn print_phrases(num_phrases: usize) {
    for i in (0..num_phrases).rev() {
        println!("{}", PHRASES[i]);
    }
}
