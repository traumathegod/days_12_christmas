use std::io;

const A_LETTERE: [[&str; 2]; 12] = [["first", ""], ["second", "Two "], ["third", "Three "],
    ["fourth", "Four "], ["fifth", "Five "], ["sixth", "Six "], ["seventh", "Seven "],
    ["eighth", "Eight "], ["ninth", "Nine "], ["tenth", "Ten "], ["eleventh", "Eleven "],
    ["twelfth", "Twelve "]];

const REGALI: [&str; 12] = ["A partridge in a pear tree", "turtle doves, ", "French hens, ",
    "calling birds, ", "gold rings, ", "geese a laying, ", "swans a swimming, ", "maids a milking, ",
    "drummers drumming, ", "pipers piping, ", "ladies dancing, ", "Lords a leaping, "];

fn main() {
    println!("PREMERE ENTER PER CONTINUARE");
    let mut n: String = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line");

    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎅🎅🎄🎄 The Twelve Days Of Christmas - Roger Whittaker 🎄🎄🎅🎅");
    println!("🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅");
    println!();

    ugola();
}

fn ugola() {
    let mut grower:String = String::from("");
    let mut g2: String;
    let mut counter: usize = 0;
    for elements in A_LETTERE.iter() {
        println!("🎁 On the {} day of Christmas my true love sent me 🎁", elements[0]);
        g2 = elements[1].to_owned() + REGALI[counter];
        grower = g2 + &grower;
        println!("{}", grower);
        counter += 1;
    }
    println!("🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄🎄");
    println!("🎅🎅🎄🎄🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎄🎄🎅🎅");
    println!("🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔JINGLE BELLS🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔🔔");
    println!("🎅🎅🎄🎄🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎄🎄🎅🎅");
    println!("🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅🎅");
}
