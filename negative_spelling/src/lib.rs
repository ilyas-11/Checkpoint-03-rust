pub fn negative_spell(n: i64) -> String {
    if n >= 0 {
        return "error: positive number".to_string();
    }

    format!("minus {}", spell((-n) as u64))
}

fn spell(n: u64) -> String {
    let small = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    match n {
        0..=19 => small[n as usize].to_string(),

        20..=99 => {
            let t = n / 10;
            let u = n % 10;

            if u == 0 {
                tens[t as usize].to_string()
            } else {
                format!("{}-{}", tens[t as usize], small[u as usize])
            }
        }

        100..=999 => {
            let h = n / 100;
            let r = n % 100;

            if r == 0 {
                format!("{} hundred", spell(h))
            } else {
                format!("{} hundred {}", spell(h), spell(r))
            }
        }

        1000..=999_999 => {
            let th = n / 1000;
            let r = n % 1000;

            if r == 0 {
                format!("{} thousand", spell(th))
            } else {
                format!("{} thousand {}", spell(th), spell(r))
            }
        }

        1_000_000 => "one million".to_string(),

        _ => String::new(),
    }
}