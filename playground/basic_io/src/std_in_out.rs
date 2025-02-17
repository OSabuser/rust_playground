use std::io::{BufRead, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut buf = String::new();

    loop {
        let _ = stdin.read_line(&mut buf).unwrap();

        let stdout = std::io::stdout();
        let mut locked_stdout = stdout.lock();
        locked_stdout
            .write_all("Your input was: ".as_bytes())
            .unwrap();
        locked_stdout.write_all(buf.as_bytes()).unwrap();

        buf.clear();
    }
}
