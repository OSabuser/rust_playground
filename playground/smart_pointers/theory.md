# Умные указатели

> RAII - Resource Acquisition Is Initialization.
Инициализация ресурса происходит в конструкторе, освобождение - в деструкторе, при выходе из области видимости.

```rust
{
    let string = String::from("Hello"); // инициализация ресурса
    println!("{}", string); // использование ресурса
}
// Освобождение ресурса (drop)
```

1. Концепция владения и перемещения (ownership and move) гарантирует, что объект всегда можно безопасно деаллоцировать по выходу из области видимости;
2. Невозможно получить частично инициализированный объект;
3. Вызов drop автоматический, но не гарантирован;

## Стандартная библиотека

### Box

Умный указатель, владеющий данными на **куче**.

```rust
let mut a = Box::new(Point { x: 1, y: 2 });
let b = Point { x: 1, y: 2 };
assert_eq!(*a, b); // Оператор * разыменовывает указатель
a.x = 3; // Оператор . разыменовывает автоматически
assert_ne!(*a, b);
// При выходе из области видимости память будет освобождена
```

- Box оборачивает любой другой тип и размещает значение этого типа на **куче**;
- Возможен как иммутабельный, так и мутабельный доступ к данным;
- Box владеет данными единолично и отвечает за их освобождение;
- Если `T` реализует `Clone`, то `Box<T>` тоже может быть клонирован;
- `Box::leak(self) -> &'static T` позволяет создать намеренную утечку памяти, чтобы продлить время жизни объекта;

#### Deref

```rust
// Возврат ссылки на Target
impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // ...
    }
}

*x; // *Deref::deref(&x)
*x = 10; // *DerefMut::deref_mut(&x)
x.foo(); // foo(&*x), если fn foo(&self)

// Возврат мутабельной ссылки на Target
impl<T> DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // ...
    }
}

```

##### Deref coercions

> Неявные преобразования типов в аргументах функций и методов.

```rust
&T -> &U where T: Deref<Target = U>
&mut T -> &mut U where T: DerefMut<Target = U>
&mut T -> &U where T: Deref<Target = U>
// T_1 -> T2 -> T_3, где
// T_1 coerces to T_2
// T_2 coerces to T_3
///
/// Пример
///
let mut map = HashMap::new(); 
let r1: &i32 = map.entry(5).or_insert(10);
let r2: &mut i32 = map.entry(10).or_insert(20);

println!("{} {}", r1, r2);

let boxed_string = Box::new(String::new());
assert_eq!(0, boxed_string.len());

// Можем вызвать метод len на boxed_string, хотя Box не обладает таким методом (а String обладает)
// Box<String> -> &String -> &str
```

### Rc

Reference Counting Smart Pointer, умный указатель на основе подсчёта ссылок.
> **Не является уникальным => на один кусок данных могут указывать несколько указателей!**

- Хранение указателя на данные в куче, а также счётчик владельцев
- Создание объекта устанавливает счётчик владельцев в 1
- Клонирования Rc увеличивает счётчик владельцев на 1
- drop(Rc) уменьшает счётчик владельцев на 1
- Объект деаллоцируется, как только счётчик владельцев станет равен 0

```rust
use std::rc::Rc;
let a = Rc::new(State { x: 1, y: 2 });
let b = Rc::clone(&a);

assert_eq!(a.x, 1);
assert_eq!(b.x, 1);

```

#### Interior Mutability

`Interior mutability` - это возможность изменить внутреннее состояние объекта, без изменения его внешнего состояния, т.е. возможность получения экслюзивного (мутабельного) доступа по разделяемой ссылке.

### Cell

Работает на стеке.

```rust

use std::cell::Cell;

let x = Cell::new(1);
let y = Cell::new(2);
let z = Cell:new(Some(1));

assert_eq!(x.get(), 1);     // get(&self) -> T: Copy
x.set(2);                   // set(&self, value: T)
y.swap(&z);                 // swap(&self, other: &Cell<T>)
let a = y.replace(0);       // replace(&self, value: T) -> T
let b = z.take();           // take(&self) -> T: Default

```

### RefCell

```rust
use std::cell::RefCell;

let x = RefCell::new(NonClonable { x: 1 });

let x_ref = x.borrow().value; //borror(&self) -> Ref<'_, T>
let x_ref2 = x.borr

```

### Mutex

### Arc
