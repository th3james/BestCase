fn best_case(input: &str) -> String {
    let mut best_cased = input.chars().fold("".to_string(), |mut res, c| {
        let chr = c.to_uppercase().collect::<String>();
        res.push_str(chr.as_str());
        res.push(' ');
        res
    });
    let length_without_space = best_cased.len() - 1;
    best_cased.truncate(length_without_space);
    best_cased
}

fn main() {
    println!(
        "{}", best_case("Returns the first element of a slice, or None if it is empty.")
    );
}

#[cfg(test)]
mod tests {
    use super::best_case;

    #[test]
    fn hello() {
        assert_eq!("H E L L O".to_string(), best_case("Hello"));
    }

    #[test]
    fn sup() {
        assert_eq!("S U P".to_string(), best_case("Sup"));
    }

    #[test]
    fn hello_world() {
        assert_eq!("H E L L O W O R L D".to_string(), best_case("Hello World"));
    }
}
