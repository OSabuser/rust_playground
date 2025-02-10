fn main() {
    let mut doc = Document::default();
}

// Имплементация трейта Default (для каждого поля вызывает ::default() )
//#[derive(Default)]
struct Document {
    name: String,
    content: Vec<u8>,
}

/// Создание структуры по умолчанию (общепринятый паттерн)
impl Default for Document {
    fn default() -> Document {
        Document {
            name: String::from("Untitled"),
            content: Vec::new(),
        }
    }
}

impl Document {
    fn new(name: &str) -> Document {
        Document {
            name: String::from(name),
            content: Vec::new(),
        }
    }
}
