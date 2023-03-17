use std::collections::HashMap;

enum Tag {
    // REGEX
    ID,

    // Reserved words
    Product,
    Epsilon,
}

struct Lexer {
    words: HashMap,
    iter: Iterator
}

impl Lexer {
    // "Constructor"
    fn new() -> Lexer{
        // Define words
        words = HashMap::from([
            ("->", Tag::Product), 
            ("' '", Tag::Epsilon)
        ])
    }

    fn scan() {
        
    }
}