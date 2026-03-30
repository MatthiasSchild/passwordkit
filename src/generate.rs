use rand::{Rng, RngExt};

const CHARS_LOWER_CASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
const CHARS_UPPER_CASE_LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARS_DIGITS: &str = "0123456789";
const CHARS_SPECIAL: &str = "!\"#$%&’()*+,-./:;<=>?@[\\]^_`{|}~";

#[derive(Default)]
pub struct Options {
    disable_lower_case: Option<bool>,
    min_lower_case: Option<u16>,
    disable_upper_case: Option<bool>,
    min_upper_case: Option<u16>,
    disable_digits: Option<bool>,
    min_digits: Option<u16>,
    disable_special_chars: Option<bool>,
    min_special_chars: Option<u16>,
    special_chars: Option<String>,
    exclude_chars: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Error {
    PasswordLength,
    MinimumsTooHigh,
    EverythingDisabled,
}

fn build_special_chars_set(options: &Options) -> String {
    let special_chars = match &options.special_chars {
        Some(special_chars) => special_chars.clone(),
        None => CHARS_SPECIAL.to_string(),
    };

    if let Some(exclude) = &options.exclude_chars {
        return special_chars
            .chars()
            .filter(|c| !exclude.contains(*c))
            .collect();
    }

    special_chars
}

fn next_random_char(rng: &mut rand::rngs::ThreadRng, set: &str) -> char {
    let chars: Vec<char> = set.chars().collect();
    let pos = rng.next_u32() as usize % chars.len();
    chars[pos]
}

pub fn generate(password_length: u16, options: Options) -> Result<String, Error> {
    if password_length == 0 {
        return Err(Error::PasswordLength);
    }

    let mut buffer: Vec<char> = vec![];
    let mut chars_set = String::new();
    let mut rng = rand::rng();

    // Lower case letters
    if !options.disable_lower_case.unwrap_or_default() {
        let min_lower_case = options.min_lower_case.unwrap_or_default();
        for _ in 0..min_lower_case {
            let next = next_random_char(&mut rng, CHARS_LOWER_CASE_LETTERS);
            buffer.push(next);
        }
        chars_set += CHARS_LOWER_CASE_LETTERS;
    }

    // Upper case letters
    if !options.disable_upper_case.unwrap_or_default() {
        let min_upper_case = options.min_upper_case.unwrap_or_default();
        for _ in 0..min_upper_case {
            let next = next_random_char(&mut rng, CHARS_UPPER_CASE_LETTERS);
            buffer.push(next);
        }
        chars_set += CHARS_UPPER_CASE_LETTERS;
    }

    // Digits
    if !options.disable_digits.unwrap_or_default() {
        let min_digits = options.min_digits.unwrap_or_default();
        for _ in 0..min_digits {
            let next = next_random_char(&mut rng, CHARS_DIGITS);
            buffer.push(next);
        }
        chars_set += CHARS_DIGITS;
    }

    // Special chars
    if !options.disable_special_chars.unwrap_or_default() {
        let special_chars_set = build_special_chars_set(&options);
        let min_special_chars = options.min_special_chars.unwrap_or_default();
        for _ in 0..min_special_chars {
            let next = next_random_char(&mut rng, &special_chars_set);
            buffer.push(next);
        }
        chars_set += &special_chars_set;
    }

    // Check if the minimums exceed the required length
    if buffer.len() > password_length as usize {
        return Err(Error::MinimumsTooHigh);
    }

    // Fill the buffer until the required password length is met
    for _ in 0..(password_length as usize - buffer.len()) {
        let next = next_random_char(&mut rng, &chars_set);
        buffer.push(next);
    }

    // Randomize the order and create a string
    let mut result: Vec<char> = vec![];
    while buffer.len() > 0 {
        let pos: usize = rng.random::<u64>() as usize % buffer.len();
        result.push(buffer.remove(pos));
    }

    Ok(result.iter().collect())
}

#[cfg(test)]
mod tests {
    use crate::{PasswordPolicy, Requirement, check};

    use super::*;

    #[test]
    fn test_generate() {
        let policy = PasswordPolicy::new_with_special_chars(
            vec![
                Requirement::MinLowerCaseLetter(2),
                Requirement::MinUpperCaseLetter(2),
                Requirement::MinDigits(2),
                Requirement::MinSpecialChars(2),
            ],
            CHARS_SPECIAL.to_string(),
        );

        // Generate 10.000 passwords and check if any error occurs
        for _ in 0..10000 {
            let password = generate(
                12,
                Options {
                    min_lower_case: Some(2),
                    min_upper_case: Some(2),
                    min_digits: Some(2),
                    min_special_chars: Some(2),
                    ..Default::default()
                },
            );
            let password = match password {
                Ok(password) => password,
                Err(err) => panic!("Failed to generate: {:?}", err),
            };

            if let Err(err) = check(&policy, &password) {
                panic!("Password {} failed policy test {:?}", password, err);
            }
        }
    }
}
