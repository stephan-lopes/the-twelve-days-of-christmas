fn main() {
    let days = ["first", "second","third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twenth"];
    let gifts = ["partridge in a pear tree", "turtle doves", "French hens","calling birds", "golden rings", "geese a-laying", "swans a-swimming", "maids a-milking", "ladies dancing", "lords a-leaping", "pipers piping", "drummers drumming"];
    let number = ["a", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"];
    println!("🎵 The Twelve Days Of Christmas 🎵");
    println!("----------------------------------");
    for (index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas,", day);
        println!("my true love sent to me");
        for gift_index in (0..index+1).rev() {
            if gift_index == 1 {
                println!("{} {}", number[gift_index], gifts[gift_index]);
                print!("And ");
            } else if gift_index > 0 {
                println!("{} {},", number[gift_index], gifts[gift_index]);
            }
        }
        println!("{} {}.", number[0], gifts[0]);
        println!("");
    }
}
