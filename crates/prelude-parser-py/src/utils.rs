pub fn to_snake(camel_string: &str) -> String {
    let mut snake_string = String::with_capacity(
        camel_string.len() + camel_string.chars().filter(|c| c.is_uppercase()).count(),
    );

    let mut chars = camel_string.chars().peekable();
    while let Some(c) = chars.next() {
        snake_string.push(c);
        if let Some(next) = chars.peek() {
            if next.is_uppercase() && c != '_' {
                snake_string.push('_');
            }
        }
    }

    snake_string.to_lowercase()
}
