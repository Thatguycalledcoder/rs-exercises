fn main() {
    for day in 1..=12 {
        a_carol(day);
    }
}

fn a_carol(mut day: i32) {
    // Get the day out of the twelve days
    let dayth = match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "Echoke",
    };

    let mut lines = String::new(); 

    // Recursively get the lines for the rest of the carol for each given day
    loop {
        let carol: &str = match day {
            1 => "a partridge in a pear tree.\n",
            2 => "two turtle doves,\n",
            3 => "three french hens,\n",
            4 => "four calling birds,\n",
            5 => "five onion rings,\n",
            6 => "six geese a laying,\n",
            7 => "seven swans a swimming,\n",
            8 => "eight elephants eating,\n",
            9 => "nine knights fighting,\n",
            10 => "ten trumpets shining,\n",
            11 => "eleven buckets dripping,\n",
            12 => "twelve tigers roaring,\n",
            _ => "Song end"
        };

        if lines.len() > 0 && day == 1 {
            lines.push_str("and ");
        }
        lines += carol;

        if day == 1 {
            break
        }

        day -= 1;
    }

    println!("On the {dayth} day of Christmas my true love sent to me:\n{lines}");
}