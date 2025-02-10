use std::fmt::{self, Formatter};

/// Если тип копируемый, но большой
/// Например, содержит поле [0; 1024]
/// Предпочтительно реализовать Clone
fn main() {
    // Перемещение и копирование (неявное для простых типов) - реализован трейт Copy
    let a = 42;
    foo(a);
    println!("It's a number: {}", a);

    // let string = String::from("Hello");
    // foo(string.clone());
    // println!("It's a string: {}", string);

    let b = MyPodType::default();
    foo(b);
    println!("It's a struct: {:?}", b);

    let c = Document {
        title: String::from("title"),
        content: vec![1, 2, 3],
    };
    foo(c.clone());
    println!("It's a struct: {:?}", c);
}

fn foo<T>(t: T) {
    //TODO:
}

#[derive(Default, Debug, Copy, Clone)]
/// Структура. содержащая копируемые, простые типы (Можно реализовать Copy)
/// Copy происходит на стеке
/// Clone - глубокое копирование
/// POD - plain old data - тип содержащий данные только на стеке
struct MyPodType<'ref_lifetime> {
    a: i32,
    b: bool,
    c: char,
    d: f32,
    e: &'ref_lifetime str,
    //f: String,
}

/// Not PodType
#[derive(Debug)]
struct Document {
    title: String,
    content: Vec<u8>,
}

impl Clone for Document {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            content: self.content.clone(),
        }
    }
}
