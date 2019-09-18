pub mod string_functions {

    /// Starting and index position take a slice of a string for x characterts
    ///
    /// # Examples
    ///
    /// ```
    /// let alpha = "abcdefg";
    /// let def = string_functions::sub_string(alpha, 4, 3);
    /// 
    /// assert_eq!(def, "def");
    /// ```
    pub fn sub_string(text: &str, start: usize, length : usize) -> &str
    {
        &text[start..(start + length)]
    }
}