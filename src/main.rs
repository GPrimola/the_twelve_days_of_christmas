fn main() {
    let christmas_days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    for christmas_day in christmas_days.iter() {
        let day = christmas_days.iter().position(|&d| christmas_day == &d ).unwrap();
        println!("On the {0} day of Christmas, my true love gave me {1}", christmas_day, presents_of(day));
    }
}

fn presents_of(mut day: usize) -> String {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five golden rings",
        "six geese a-layin'",
        "seven swans a-swimmin'",
        "eight maids a-milkin'",
        "nine lords a-leapin'",
        "ten ladies dancin'",
        "eleven pipers pipin'",
        "twelve drummers drummin'"
    ];

    let mut presents = String::new();

    while day > 0 {
	presents = presents + gifts[day];
        
        if day == 1 {
            presents = presents + " and ";
        } else {
            presents = presents + ", ";
        }

	day -= 1;
    }
    
    presents + gifts[0]
}
