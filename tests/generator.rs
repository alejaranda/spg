use spg::generator::generate_password;

const LETTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>?/";

#[test]
fn password_has_correct_length() {
    let pwd = generate_password(16, true);
    assert_eq!(pwd.len(), 16);
}

#[test]
fn zero_length_returns_empty_string() {
    let pwd = generate_password(0, true);
    assert!(pwd.is_empty());
}

#[test]
fn no_symbols_when_disabled() {
    let pwd = generate_password(100, false);

    for c in pwd.chars() {
        assert!(
            LETTERS.contains(c),
            "unexpected symbol found in password: {c}"
        );
    }
}

#[test]
fn only_valid_characters_when_symbols_enabled() {
    let pwd = generate_password(100, true);

    for c in pwd.chars() {
        assert!(
            LETTERS.contains(c) || SYMBOLS.contains(c),
            "invalid character generated: {c}"
        );
    }
}

#[test]
fn symbols_can_appear_when_enabled() {
    // try multiple times since generation is random
    let mut found = false;

    for _ in 0..100 {
        let pwd = generate_password(64, true);
        if pwd.chars().any(|c| SYMBOLS.contains(c)) {
            found = true;
            break;
        }
    }

    assert!(found, "expected at least one password to contain symbols");
}

#[test]
fn symbols_never_appear_when_disabled() {
    for _ in 0..50 {
        let pwd = generate_password(64, false);

        assert!(
            !pwd.chars().any(|c| SYMBOLS.contains(c)),
            "symbol generated even though symbols are disabled"
        );
    }
}
