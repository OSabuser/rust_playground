enum Status {
    Ok,
    ErrorCode(ErrorCode),
}
enum ErrorCode {
    NotAvaiable,
    Disconnected
}

struct Template {
    name: String,
    status: Status
}

fn show_string(string: String) {
    println!("{}", string);
}


fn main() {

    let new_string = String::from("Hello!");
    show_string(new_string);
    println!("{new_string}");
}
