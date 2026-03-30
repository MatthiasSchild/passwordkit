use std::fmt::Display;

const DEFAULT_SPECIAL_CHARS: &str = " !\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

#[derive(Debug, Clone)]
pub enum Requirement {
    MinLength(u16),
    MaxLength(u16),
    MinLowerCaseLetter(u16),
    MaxLowerCaseLetter(u16),
    MinUpperCaseLetter(u16),
    MaxUpperCaseLetter(u16),
    MinDigits(u16),
    MaxDigits(u16),
    MinSpecialChars(u16),
    MaxSpecialChars(u16),
}

impl Display for Requirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Requirement::MinLength(v) => {
                write!(f, "password should have a minimum length of {v}")
            }
            Requirement::MaxLength(v) => {
                write!(f, "password should have a maximum length of {v}")
            }
            Requirement::MinLowerCaseLetter(v) => {
                write!(f, "password should have at least {v} lower case letters")
            }
            Requirement::MaxLowerCaseLetter(v) => {
                write!(f, "password should have at most {v} lower case letters")
            }
            Requirement::MinUpperCaseLetter(v) => {
                write!(f, "password should have at least {v} upper case letters")
            }
            Requirement::MaxUpperCaseLetter(v) => {
                write!(f, "password should have at most {v} upper case letters")
            }
            Requirement::MinDigits(v) => {
                write!(f, "password should have at least {v} digits")
            }
            Requirement::MaxDigits(v) => {
                write!(f, "password should have at most {v} digits")
            }
            Requirement::MinSpecialChars(v) => {
                write!(f, "password should have at least {v} special characters")
            }
            Requirement::MaxSpecialChars(v) => {
                write!(f, "password should have at most {v} special characters")
            }
        }
    }
}

pub struct PasswordPolicy {
    requirements: Vec<Requirement>,
    special_chars: String,
}

impl PasswordPolicy {
    pub fn new(requirements: Vec<Requirement>) -> Self {
        Self {
            requirements,
            special_chars: DEFAULT_SPECIAL_CHARS.to_string(),
        }
    }

    pub fn new_with_special_chars(requirements: Vec<Requirement>, special_chars: String) -> Self {
        Self {
            requirements,
            special_chars,
        }
    }
}

pub fn check(policy: &PasswordPolicy, password: &str) -> Result<(), Vec<Requirement>> {
    let mut failed_requirements: Vec<Requirement> = vec![];
    let mut count_lower_case: u16 = 0;
    let mut count_upper_case: u16 = 0;
    let mut count_digits: u16 = 0;
    let mut count_special_chars: u16 = 0;

    for c in password.chars() {
        if c.is_lowercase() {
            count_lower_case += 1;
            continue;
        }
        if c.is_uppercase() {
            count_upper_case += 1;
            continue;
        }
        if c.is_numeric() {
            count_digits += 1;
            continue;
        }
        if policy.special_chars.contains(c) {
            count_special_chars += 1;
            continue;
        }
    }

    for req in &policy.requirements {
        match req {
            Requirement::MinLength(min_length) => {
                if password.len() < *min_length as usize {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MaxLength(max_length) => {
                if password.len() > *max_length as usize {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MinLowerCaseLetter(min_lower_case) => {
                if count_lower_case < *min_lower_case {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MaxLowerCaseLetter(max_lower_case) => {
                if count_lower_case > *max_lower_case {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MinUpperCaseLetter(min_upper_case) => {
                if count_upper_case < *min_upper_case {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MaxUpperCaseLetter(max_upper_case) => {
                if count_upper_case > *max_upper_case {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MinDigits(min_digits) => {
                if count_digits < *min_digits {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MaxDigits(max_digits) => {
                if count_digits > *max_digits {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MinSpecialChars(min_symbols) => {
                if count_special_chars < *min_symbols {
                    failed_requirements.push(req.clone());
                }
            }
            Requirement::MaxSpecialChars(max_symbols) => {
                if count_special_chars > *max_symbols {
                    failed_requirements.push(req.clone());
                }
            }
        }
    }

    if !failed_requirements.is_empty() {
        return Err(failed_requirements);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_min_length() {
        let policy = PasswordPolicy::new(vec![Requirement::MinLength(8)]);

        let result_too_short = check(&policy, "1234567");
        assert!(result_too_short.is_err());

        let result_short_enough = check(&policy, "12345678");
        assert!(result_short_enough.is_ok());
    }

    #[test]
    fn test_max_length() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxLength(12)]);

        let result_long_enough = check(&policy, "123456789012");
        assert!(result_long_enough.is_ok());

        let result_too_long = check(&policy, "1234567890123");
        assert!(result_too_long.is_err());
    }

    #[test]
    fn test_min_lower_case_letter() {
        let policy = PasswordPolicy::new(vec![Requirement::MinLowerCaseLetter(3)]);

        let result_too_few = check(&policy, "ABcd");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "ABCdef");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_lower_case_letter() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxLowerCaseLetter(3)]);

        let result_ok = check(&policy, "ABCdef");
        assert!(result_ok.is_ok());

        let result_too_many = check(&policy, "ABCdefgh");
        assert!(result_too_many.is_err());
    }

    #[test]
    fn test_min_lower_case_letters_unicode() {
        let policy = PasswordPolicy::new(vec![Requirement::MinLowerCaseLetter(8)]);

        let result_too_few = check(&policy, "привет");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "привет, мир");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_lower_case_letters_unicode() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxLowerCaseLetter(8)]);

        let result_too_many = check(&policy, "привет, мир");
        assert!(result_too_many.is_err());

        let result_enough = check(&policy, "привет");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_min_upper_case_letter() {
        let policy = PasswordPolicy::new(vec![Requirement::MinUpperCaseLetter(3)]);

        let result_too_few = check(&policy, "ABcd");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "ABCdef");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_upper_case_letter() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxUpperCaseLetter(3)]);

        let result_ok = check(&policy, "ABCdef");
        assert!(result_ok.is_ok());

        let result_too_many = check(&policy, "ABCDEFgh");
        assert!(result_too_many.is_err());
    }

    #[test]
    fn test_min_upper_case_letters_unicode() {
        let policy = PasswordPolicy::new(vec![Requirement::MinUpperCaseLetter(2)]);

        let result_too_few = check(&policy, "Привет");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "Привет, Мир");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_upper_case_letters_unicode() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxUpperCaseLetter(1)]);

        let result_too_many = check(&policy, "Привет, Мир");
        assert!(result_too_many.is_err());

        let result_enough = check(&policy, "Привет");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_min_digits() {
        let policy = PasswordPolicy::new(vec![Requirement::MinDigits(3)]);

        let result_too_few = check(&policy, "abc12");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "abc123");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_digits() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxDigits(3)]);

        let result_ok = check(&policy, "abc123");
        assert!(result_ok.is_ok());

        let result_too_many = check(&policy, "abc1234");
        assert!(result_too_many.is_err());
    }

    #[test]
    fn test_min_special_chars() {
        let policy = PasswordPolicy::new(vec![Requirement::MinSpecialChars(3)]);

        let result_too_few = check(&policy, "abc!@");
        assert!(result_too_few.is_err());

        let result_enough = check(&policy, "abc!@#");
        assert!(result_enough.is_ok());
    }

    #[test]
    fn test_max_special_chars() {
        let policy = PasswordPolicy::new(vec![Requirement::MaxSpecialChars(3)]);

        let result_ok = check(&policy, "abc!@#");
        assert!(result_ok.is_ok());

        let result_too_many = check(&policy, "abc!@#$");
        assert!(result_too_many.is_err());
    }

    #[test]
    fn test_custom_special_chars() {
        let policy = PasswordPolicy::new_with_special_chars(
            vec![Requirement::MinSpecialChars(2)],
            "!@#".to_string(),
        );

        let result_with_default_specials = check(&policy, "abc$%");
        assert!(result_with_default_specials.is_err());

        let result_with_custom_specials = check(&policy, "abc!@");
        assert!(result_with_custom_specials.is_ok());
    }

    #[test]
    fn test_multiple_requirements() {
        let policy = PasswordPolicy::new(vec![
            Requirement::MinLength(8),
            Requirement::MinUpperCaseLetter(1),
            Requirement::MinLowerCaseLetter(1),
            Requirement::MinDigits(1),
            Requirement::MinSpecialChars(1),
        ]);

        let result_fail_all = check(&policy, "abc");
        assert!(result_fail_all.is_err());
        assert_eq!(result_fail_all.unwrap_err().len(), 4);

        let result_fail_some = check(&policy, "Abcdefgh");
        assert!(result_fail_some.is_err());
        assert_eq!(result_fail_some.unwrap_err().len(), 2);

        let result_pass = check(&policy, "Abcdef1!");
        assert!(result_pass.is_ok());
    }
}
