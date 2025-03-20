fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let first_non_white = get_first_non_whitespace_index(input);
    let last_non_white = get_last_non_whitespace_index(input) + 1;

    &input[first_non_white..last_non_white]
}

fn get_last_non_whitespace_index(input: &str) -> usize {
    let mut current_best_guess: usize = 0;
    for (i, c) in input.chars().enumerate() {
        if !is_whitespace(c) {
            current_best_guess = i;
        }
    }

    current_best_guess
}

fn get_first_non_whitespace_index(input: &str) -> usize {
    for (i, c) in input.chars().enumerate() {
        if !is_whitespace(c) {
            return i;
        }
    }

    0
}

fn is_whitespace(input: char) -> bool {
    input == ' ' || input == '\t'
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut input = String::from(input);
    input.push_str(" world!");
    input
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let input = String::from(input);
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
