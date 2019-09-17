// This is where integration tests live.
use rust_samples::string_functions::sub_string;

#[test]
fn compare_size_with_lifetimes_succeds() {
    let text = "Asante sana squash banana, wewe nugu mimi hapana";

    assert_eq!("squash banana", sub_string(text, 12, 13));
}

#[test]
fn compare_size_with_lifetimes_zero_zero() {
    let text = "Asante sana squash banana, wewe nugu mimi hapana";

    assert_eq!("", sub_string(text, 0, 0));
}

#[test]
#[should_panic(expected = "out of bounds")]
fn compare_size_with_lifetimes_overflow() {
    let text = "Asante sana squash banana, wewe nugu mimi hapana";

    sub_string(text, 0, 150);
}