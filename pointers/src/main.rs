// Функция получает аргумент пароля T, где в T реализуется AsRef<str>
// Средства реализации AsRef<str> ведут себя как &str, даже если это не соответствует действительности
// Такой приём используется, когда требуется доступ только по чтению
fn is_password_strong <T: AsRef<str>> (password: T) -> bool {
    password.as_ref().len() > 5
}

// Если доступ требуется и по чтению, и по записи, используется типаж AsMut<T>
fn main() {
    let permission = is_password_strong(String::from("Hel:13lo#"));

    assert!(permission);
}