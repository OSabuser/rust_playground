# Стандартные коллекции RUST

> [Официальная документация](https://doc.rust-lang.org/std/collections/)

Находятся в модуле `std::collections`  

- Последовательности: `Vec, VecDeque, LinkedList`

    Эффективное добавление элемента в начало и конец, итерирование + известен порядок элементов.

- Словари: `HashMap, BTreeMap, HashSet, BTreeSet`

    Эффективное чтение по ключу

- Множества: `HashSet, BTreeSet`

- Прочее: `BinaryHeap`

## Общий интерфейс

1. `new()` - создает пустую коллекцию, без выделения памяти
2. `with_capacity(n: size) -> Self` - аллокация под ожидаемое кол-во элементов заранее
3. `len(&self) -> usize`
4. `is_empty(&self) -> bool`
5. `clear(&mut self)` - удаляет элементы (len = 0), capacity не изменяется!
6. `get(), get_mut()` `-> Option<&V>, Option<&mut V>`

### Vec

- Динамический массив с быстрой вставкой в конец `O(1)`
- Может работать как стек, `push(&mut self, T), pop(&mut self) -> Option<T>`
- `insert(&mut self, index: usize, element: T)` - вставляет элемент в определенную позицию `O(n)`
- `remove(&mut self, index: usize) -> T` - удаляет элемент по индексу `O(n)`
- `swap_remove(&mut self, index: usize)` - удаление по индексу, в случае если порядок элементов неважен
- `sort(&mut self), sort_unstable(&mut self)` - `stable` - не меняет порядок равных элементов, но медленнее `unstable`
- vec![] - макрос для быстрого конструирования вектора
- Реализует `Deref<Target = [T]>`

```rust
// Структура Vec (размещается на стеке)
struct Vec<T> {
    ptr: NonNull<T>, // Указатель на первый элемент в куче
    len: usize, // Количество элементов 
    capacity: usize, // Размер выделенной памяти
}
```

> Если при добавлении элемента в вектор превышается велчина `capacity`, происходит реалокация вектора.
Создаётся большая область памяти, в которую копируются содержимое вектора. Старая область памяти деалоцируется.

### VecDeque

- Динамический массив с быстрой вставкой в начало и конец
- `push_back(&mut self, T)`
- `push_front(&mut self, T)`
- `pop_back(&mut self) -> Option<T>`
- `pop_front(&mut self) -> Option<T>`
- `make_contigious(&mut self)` - хранение вектора в одной непрерывной области памяти (для преобразования в слайс)

#### String

- По сути `Vec<u8>`
- Внутри обязательно корректный `UTF-8`
- `len()` - возвращает количество байт, а не символов строки!
- Нет доступа к конкретному байту, но есть доступ к последовательности байт `[0..3]`
- `push_str(&mut self, s: &str)` - конкатенация строки

    ```rust
    // Второй способ конкатенации строк
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    // !! `s1` больше использовать нельзя!

    // Конкатенация строковых литералов
    // Одно из слагаемых должно быть строкой!
    let a = "some str1";
    let b = "some str2";
    let c = a.to_string() + b;
    ```

- `format!("{}{}", s1, s2)` - **копирование** `s1` и `s2`
- `[s1, s2].concat()` - **копирование** `s1` и `s2`

### HashMap

> Структура данных "КЛЮЧ - ЗНАЧЕНИЕ"

- Словарь (ассоциативный массив)
- Тип ключа должен реализовывать `Eq` и `Hash`
- `insert(&mut self, key: K, value: V)`
- `remove(&mut self, key: &K) -> Option<V>`
- Можно менять используемый хеш: `with_hasher(hasher: H)`
- EntryAPI

#### Хеширование

> Преобразование значения типа `T` в `u64`

```rust
trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized;
}
```

#### EntryAPI

[entryAPI](entryAPI.png)

```rust

// Неэффективный подсчёт количества символов в строке
// В худшем случае в одной итерации цикла поиск под ключу будет осуществляться 3 раза
let string = "Let's count character in this string";
let mut map = HashMap::new();

for c in string.chars() {
    if map.contains_key(&c) {
        let count = *map.get_mut(&c).unwrap();
        map.insert(c, count + 1);
    } else {
        map.insert(c, 1);
    }
}

// Альтернативный способ
// В результате выполнения [1] получаем либо ссылку на существующий элемент, либо новый элемент со значением 0
let string = "Let's count character in this string";
let mut map = HashMap::new();

for c in string.chars() {
    let mut count = map.entry(c).or_insert(0); // [1]
    *count += 1;
}
```  

#### Строковые ключи

```rust
let mut distance: HashMap<&str, f64> = HashMap::from([
    ("Mercury", 0.4),
    ("Venus", 0.7),
    ("Earth", 1.0),
    ("Mars", 1.5),
    ("Jupiter", 5.2),
]);

{
    let jupiter = String::from("Jupiter");
    distance.insert(&jupiter, 5.2); // ?? Проблема с временем жизни!
}

// Другой вариант:

let mut distance: HashMap<String, f64> = HashMap::from([
    ("Mercury".to_string(), 0.4),
    ("Venus".to_string(), 0.7),
    ("Earth".to_string(), 1.0),
    ("Jupiter".to_string(), 5.2),
]);
{
    distance.insert("Mars".to_string(), 1.5);
}

let earth_distance = distance.get("Earth"); // Можно использовать &str!

```

### HashSet & BTreeSet

Официальная документация: [HashSet](https://doc.rust-lang.org/std/collections/struct.HashSet.html), [BTreeSet](https://doc.rust-lang.org/std/collections/struct.BTreeSet.html)

> У `BTreeSet` по умолчанию отсортированы ключи

```rust

type HashSet<T> = HashMap<T, ()>;
type BTreeSet<T> = BTreeMap<T, ()>;

```
