# passwordkit

This Rust library is a tiny tool kit for working with passwords.

## Generate passwords

To generate a password, you can use the `generate` function.
You need to enter the desired password length and options to the function.
The options specifies how the result should look like.

```
fn main() {
    let result = passwordkit::generate(
        12,
        Options {
            min_lower_case: Some(2),
            min_upper_case: Some(2),
            min_digits: Some(2),
            min_special_chars: Some(2),
            ..Default::default()
        },
    );

    let password = match result {
        Ok(password) => password,
        Err(err) => panic!("Failed to generate: {:?}", err),
    };
}
```

## Check password security
