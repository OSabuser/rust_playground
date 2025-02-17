use std::io::{BufReader, BufWriter, Cursor, Read, Write};

fn main() {
    let data = (0_u8..10).collect::<Vec<u8>>();
    let cursor = Cursor::new(&data);
    let print_read = PrintStream(cursor);

    let mut print_read = BufReader::with_capacity(4, print_read);

    let mut buf = [0_u8; 10];

    for i in 0..10 {
        print_read.read(&mut buf[i..i + 1]).unwrap();
        println!("Read: {buf:?}");
    }

    let read: usize = print_read.read(&mut buf).unwrap();

    let mut output = [0_u8; 10];
    let write_cursor = Cursor::new(&mut output[..]);
    let print_write = PrintStream(write_cursor);
    let mut print_write = BufWriter::with_capacity(2, print_write);

    for i in 0..10 {
        print_write.write(&buf[i..i + 1]).unwrap();
        println!("Wrote: {buf:?}");
    }

    // BufWriter drops here
}
struct PrintStream<T>(T);

impl<T: Write> Write for PrintStream<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        println!("Your input was: {buf:?}");
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        println!("Flushing...");
        self.0.flush()
    }
}

impl<T: Read> Read for PrintStream<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let result = self.0.read(buf);
        println!("Reading...{buf:?}");
        result
    }
}
