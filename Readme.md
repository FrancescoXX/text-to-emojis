# Text to Emoji

A Rust library to convert words in a sentence into emojis! This library provides a fun and simple way to add emoji representations to text.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
text_to_emoji = "0.1.0"
```

### Example

```rust
use text_to_emoji::convert_to_emojis;

let result = convert_to_emojis("I love coffee and pizza");
assert_eq!(result, "I ❤️ ☕️ and 🍕");
```

## License

This project is licensed under the MIT License.
