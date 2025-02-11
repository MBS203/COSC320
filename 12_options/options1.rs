fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    if hour_of_day > 23 {
        None
    } else if hour_of_day < 22 {
        Some(5)
    } else {
        Some(0)
    }
}

fn main() {
    let hours = [10, 22, 24];

    for &hour in &hours {
        match maybe_icecream(hour) {
            Some(scoops) => println!("At {}:00, there are {} scoops of ice cream left.", hour, scoops),
            None => println!("At {}:00, invalid hour!", hour),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12).unwrap(); // Use unwrap() to get the value inside Option

        assert_eq!(icecreams, 5);
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
