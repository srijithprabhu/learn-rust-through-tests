fn hello_message(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello_world_message() {
        let message = hello_message("World");
        assert_eq!(message, "Hello, World!")
    }

    #[test]
    fn test_human_message() {
        let message = hello_message("Human");
        assert_eq!(message, "Hello, Human!")
    }
}