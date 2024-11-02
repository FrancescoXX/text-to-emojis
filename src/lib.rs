mod emoji_mappings;

/// Converts words in a given sentence to their corresponding emojis.
///
/// # Arguments
///
/// * `text` - A string slice that holds the sentence to convert
///
/// # Returns
///
/// A new `String` with matching words replaced by emojis
///
/// # Examples
///
/// ```
/// use text_to_emoji::convert_to_emojis;
///
/// let result = convert_to_emojis("I love coffee and pizza");
/// assert_eq!(result, "I ❤️ ☕️ and 🍕");
/// ```
pub fn convert_to_emojis(text: &str) -> String {
    let emoji_map = emoji_mappings::get_emoji_map();

    text.split_whitespace()
        .map(|word| emoji_map.get(word).unwrap_or(&word).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_emojis() {
        let input = "I love coffee and pizza";
        let expected = "I ❤️ ☕️ and 🍕";
        assert_eq!(convert_to_emojis(input), expected);
    }

    #[test]
    fn test_no_emojis() {
        let input = "No emojis here";
        let expected = "No emojis here";
        assert_eq!(convert_to_emojis(input), expected);
    }
}
