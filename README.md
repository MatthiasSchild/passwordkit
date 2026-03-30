# passwordkit

Small Rust helpers to generate random passwords and to check a password against a policy (length and character-class rules).

---

## Generate passwords

Call `passwordkit::generate` with the desired password length and an `Options` value. Use the fields below to require minimum counts per character class, turn classes off entirely, or adjust which special characters are allowed.

`Options` fields (all optional except what you set explicitly):

- `min_lower_case`, `min_upper_case`, `min_digits`, `min_special_chars`: at least this many characters from that class.
- `disable_lower_case`, `disable_upper_case`, `disable_digits`, `disable_special_chars`: if true, that class is not used at all.
- `special_chars`: custom string of allowed special characters for generation (default is a fixed ASCII punctuation set).
- `exclude_chars`: characters to remove from the special-character set (after `special_chars` is chosen).

Errors:

- `Error::PasswordLength` if length is zero.
- `Error::MinimumsTooHigh` if the sum of the minimums is greater than the requested length.

Example:

```rust
use passwordkit::{generate, Options};

fn main() {
    let password = generate(
        12,
        Options {
            min_lower_case: Some(2),
            min_upper_case: Some(2),
            min_digits: Some(2),
            min_special_chars: Some(2),
            ..Default::default()
        },
    )
    .expect("generate");

    println!("{}", password);
}
```

---

## Check password security

Define a `PasswordPolicy` as a list of `Requirement` values, then call `check(&policy, password)`.

`Requirement` describes rules such as minimum/maximum length, minimum/maximum counts of lower case letters, upper case letters, digits, and special characters. Special characters are only those contained in the policy’s `special_chars` string (default includes common ASCII punctuation; use `PasswordPolicy::new_with_special_chars` to set which characters count as special).

`check` returns `Ok(())` if every requirement passes. If not, it returns `Err` with a `Vec<Requirement>` listing the requirements that failed (the same enum values you put in the policy, so you can match on them or print them; `Requirement` implements `Display` with short human-readable messages).

Example:

```rust
use passwordkit::{check, PasswordPolicy, Requirement};

fn main() {
    let policy = PasswordPolicy::new(vec![
        Requirement::MinLength(8),
        Requirement::MinLowerCaseLetter(1),
        Requirement::MinUpperCaseLetter(1),
        Requirement::MinDigits(1),
        Requirement::MinSpecialChars(1),
    ]);

    match check(&policy, "Abcdef1!") {
        Ok(()) => println!("ok"),
        Err(failed) => {
            for req in failed {
                eprintln!("{}", req);
            }
        }
    }
}
```

Unicode: letter and digit counts use Rust’s `char::is_lowercase`, `is_uppercase`, and `is_numeric`, so rules apply to Unicode letters/digits where those methods classify them.
