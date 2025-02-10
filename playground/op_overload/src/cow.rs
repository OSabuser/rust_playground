use std::borrow::Cow;

fn main() {
    let even = make_even_len("Hell");
    let not_even = make_even_len("Hello world");

    println!("{}", even);
    println!("Even is borrowed: {}", matches!(even, Cow::Borrowed(_)));
    println!("{}", not_even);
    println!(
        "Not even is borrowed: {}",
        matches!(not_even, Cow::Borrowed(_))
    );
}

fn make_even_len(s: &str) -> Cow<'_, str> {
    let mut cow = Cow::Borrowed(s);
    if s.len() % 2 == 0 {
        //s.into()
        // Возврат ссылки без изменений
        return cow;
    }
    // Borrowed to Owned
    // Копирование строки для изменения, возврат ссылки
    cow.to_mut().push('0');
    cow
}
