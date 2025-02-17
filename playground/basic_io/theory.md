# Передача параметров/данных в программу

## Переменные окружения

```rust
// Можно использовать как compile time, так и run time переменные:
fn main() {
    // Значение переменной на машине, на которой компилируется код
    const PATH: &str = std::env!("PATH");
    println!("Compile time: {}", PATH);

    // Значение переменной на машине, на которой выполняется код
    let path = std::env::var("PATH").unwrap();
    println!("Run time: {}", path);
}

```

## Аргументы командной строки

1. Парсятся с помощью [`std::env::args()`](https://doc.rust-lang.org/std/env/fn.args.html)
2. Популярный крейт: [`clap`](https://crates.io/crates/clap)

```rust
fn main() {
    let args: Vec<String> = std::env::args();
    
    for (i, arg) in args.enumerate() {
        println!("Arg#{}: {}", i, arg);
    }
}
```

```bash
cargo run "custom arg" 1234

> Arg#0: target/debug/example
> Arg#1: custom arg
> Arg#2: 1234

```

## Static данные

Можно включать файлы в бинарник в строковом или байтовом виде.

```rust
fn main() {
    let string = include_str!("../Cargo.toml");
    let bytes = include_bytes!("../Cargo.toml");
    println!("../Cargo.toml:{} with bytes len", string, bytes.len());
}
```

## Трейты и типы для I/O

Рантайм вывод данных из приложения.

### Трейты

- `Read` - входной байтовый поток данных
- `Write` - выходной байтовый поток данных
- `Seek` - перемещение в байтовом потоке
- `BufRead:Read` - буферизированное чтение

### Типы

- `std::io::Error` - ошибка операции ввода/вывода
- `ErrorKind` - вид ошибки ввода/вывода
- `Cursor: BufRead + Seek` - представляет данные в памяти в виде входного байтового потока
- `BufReader: BufRead` - буферизация входных данных
- `BufWriter` - буферизация выходных данных

#### Структура io::Error

Ошибка для любых I/O операций.

```rust
pub type Result<T> = Result<T, Error>

// Содержит пречисление ErrorKind
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    EndOfStream,
    Interrupted,
    Other,
    UnexpectedEof,
    Unexpected,
    OsError(u32),
}
```

#### Trait Read

Описывает чтение данных из источника

```rust
pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize>;
    fn read_to_string(&mut self, buf: &mut String) -> io::Result<usize>;    
    fn is_read_vectored(&self) -> bool;// Non stabilized
    fn read_vectored(&mut self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize>;
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()>;

    fn bytes(&mut self) -> Bytes<&mut Self>;
    fn chain<R: Read>(self, other: R) -> Chain<Self, R> where Self: Sized;
    fn take(self, limit: u64) -> Take<Self> where Self: Sized;
}
```

#### Trait Write

Описывает запись потока байт

```rust
trait Write {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    fn flush(&mut self) -> io::Result<()>; // Непосредственная запись в приёмник (гарантия передачи данных)
    fn write_all(&mut self, buf: &[u8]) -> io::Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> io::Result<()>;
    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> io::Result<usize>;
    fn is_write_vectored(&self) -> bool;// Non stabilized
}
```

#### Trait Seek

Навигация по потоку данных  

```rust
trait Seek {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64>;
    fn stream_position(&mut self) -> io::Result<u64>;
    fn rewind(&mut self) -> io::Result<()>;
    fn stream_len(&mut self) -> io::Result<u64>;
}
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}
```

#### Trait BufRead

Описывает чтение из источника с внутренним буфером

```rust
trait BufRead {
    fn fill_buf(&mut self) -> io::Result<&[u8]>;
    fn consume(&mut self, amt: usize);

    fn read_line(&mut self, buf: &mut String) -> io::Result<usize>;
    fn read_until(&mut self, byte: u8, buf: &mut String) -> io::Result<usize>;
    fn has_data_left(&mut self) -> Result<bool>;// Non stabilized
    fn split(&mut self) -> Split<Self> where Self: Sized;
    fn lines(&mut self) -> Lines<Self> where Self: Sized;
}
```

#### Struct Cursor

Позволяет использовать данные в памяти как BufRead объект

```rust
impl<T>Cursor<T>
where
    T: AsRef<[u8]>,

let data = b"First\r\nSecond\r\nThird\r\n";

let mut cursor = Cursor::new(data);
let lines = cursor.lines();

for (i, line) in lines.enumerate() {
    println!("{}: {}", i, line?);
}
```

## Реализация I/O

### Stdout

- Под капотом содержит общий глобальный буфер
- Каждая запись синхронизируется "из коробки"
- Позволяет взять lock и гарантировать последовательный вывод данных
- Stderr работает аналогично Stdout

```rust
io::stdout().write_all(b"Hello world!");

let mut buf = [b'x'; 10];
let written = io::stdout().write(&mut buf)?;
let output = io::stdout();
let mut lock = output.lock();
writeln!(lock, "First: {}", 1);
writeln!(lock, "Second: {}", 2);
writeln!("Third");
```
