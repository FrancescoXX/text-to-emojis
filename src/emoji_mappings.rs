use std::collections::HashMap;

pub fn get_emoji_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("happy", "😊");
    map.insert("coffee", "☕️");
    map.insert("pizza", "🍕");
    map.insert("love", "❤️");
    map.insert("sun", "☀️");
    map
}
