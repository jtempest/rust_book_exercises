const ORDINAL: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const DAY_THINGS: [&str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "FIVE GOLDEN RINGS!",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn main() {
    for d in 0..12 {
        println!(
            "On the {} day of Christmas, my true love gave to me",
            ORDINAL[d]
        );

        let things_in_song_order = DAY_THINGS[1..d + 1].iter().rev();
        for t in things_in_song_order {
            println!("{}", t);
        }

        let and = if d > 0 { "and " } else { "" };
        println!("{}{}", and, DAY_THINGS[0]);

        println!();
    }
}
