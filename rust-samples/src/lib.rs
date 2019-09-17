pub mod string_functions {

    pub fn sub_string(text: &str, start: usize, length : usize) -> &str
    {
        &text[start..(start + length)]
    }
}