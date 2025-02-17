# Итераторы

Это объекты, представляющие последовательность элементов и по которым можно проходить.
Итераторы в RUST должны реализовать трейт `Iterator`

1. Итератор - абстракция для доступа к перечисляемому объекту
2. Итератор - ленивая последовательность: создание **бесплатно**, вычисление можно запускать позже
3. `IntoIterator` - позволяет использовать типы в цикле for
4. `FromIterator` - позволяет создать коллекцию из итератора

```rust
use std::iter::Iterator;

trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // ... методы с реализацией по умолчанию
}

// Трансформация коллекции в итератор
let v = vec![1, 2, 3];
let mut iter = v.into_iter();

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next(), Some(2));
assert_eq!(iter.next(), Some(3));
assert_eq!(iter.next(), None);
```

> Если итератор возвращает `None`, то это не значит, что он закончил свою работу.
Допустимо возвращать следующие элементы итератора после возвращения `None`.

## Адаптеры

```rust
let mut iter = (1..10)
    .map(|x| x * 2)
    .filter(|x| x % 2 == 0);

// Только здесь при каждой итерации начнётся применение map & filter!
for val in iter {
    println!("Value:{}", val);
}
```

## Infinite iterators

```rust
let numbers = 0..;
let first_five: Vec<_> = numbers.take(5).collect();
```

## Useful adapters

1. Настройка итератора

- `step_by(step: usize)` - проход по итератору с заданным шагом
- `enumerate()` - возвращает пару `(index, value)`
- `skip(n: usize)` - пропускает первые n элементов
- `take(n: usize)` - возвращает первые n элементов
- `rev()` - возвращает итератор в обратном порядке
- `nth(n: usize)` - возвращает n-й элемент итератора
- `map(f)` - применяет функцию к каждому элементу итератора
- `filter(f)` - фильтрует элементы итератора

2. Использование итератора

- `find(p: &Fn(&T) -> bool) -> Option<T>` - ищет первый элемент, удовлетворяющий условию
- `position(p: &Fn(&T) -> bool) -> Option<usize>` - возвращает индекс первого элемента, удовлетворяющего условию
- `any(p: &Fn(&T) -> bool) -> bool` - возвращает true, если хотя бы один элемент удовлетворяет условию
- `all(p: &Fn(&T) -> bool) -> bool` - возвращает true, если все элементы удовлетворяют условию
- `min(p: &Fn(&T, &T) -> bool) -> Option<T>` - возвращает минимальный элемент
- `max(p: &Fn(&T, &T) -> bool) -> Option<T>` - возвращает максимальный элемент
- `sum(p: &Fn(&T, &T) -> T) -> T` - возвращает сумму элементов
- `count(p: &Fn(&T) -> bool) -> usize` - возвращает количество элементов, удовлетворяющих условию

## Transformations  

- `into_iter()` - преобразует коллекцию в итератор `Iterator<Item = T>`
- `iter()` - преобразует коллекцию в итератор `Iterator<Item = &T>`
- `iter_mut()` - преобразует коллекцию в итератор `Iterator<Item = &mut T>`

## Реализация собственного итератора  

```rust
struct MyType {
    items: Vec<String>,
}

impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        MyTypeIterator {
            index: 0,
            items: &self.items
        }
    }
} 

struct MyTypeIterator<'a> {
    index: usize,
    items: &'a Vec<String>,
}

impl<'a> Iterator for MyTypeIterator<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.items.len() {
            let item = &self.items[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }

}
```

## Переиспользвание std итератора

```rust
struct MyType {
    items: Vec<String>,
}

impl MyType {
    fn iter(&self) -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}
```

## Itertools

[itertools](https://docs.rs/itertools/latest/itertools/)

```rust
use itertools::Itertools;
let perms = (5..8).permutations(2);

itertools::assert_equal( perms, vec![
    vec![5, 6],
    vec![5, 7],
    vec![6, 5],
    vec![6, 7],
    vec![7, 5],
    vec![7, 6],
]);
```

## IntoIterator

```rust
trait IntoIterator 
where
    <Self::IntoIter as Iterator>::Item == Self::Item>
{
    type Item;
    type IntoIter: Iterator;

    fn into_iter(self) -> Self::IntoIter;
}

// Пример работы цикла (for работает, так как для вектора реализован IntoIterator)
let values = vec![1, 2, 3];

for v in values {
    println!("{}", v);
}
// Эквивалетно:
let mut iterator = IntoIterator::into_iter(values);
loop {
    let x;
    match iterator.next() {
        Some(v) => x = v,
        None => break,
    };
    println!("{}", x);
}
```

## FromIterator

```rust
trait FromIterator<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>;
}

// Iterator::collect
fn collect<B>(self) -> B
where
    B: FromIterator<Self::Item>;


// Пример 1 - получение строки с буквенными символами из произвольной
fn filter_letters(string: &str) -> String {
    string.chars().filter(|c| c.is_alphabetic()).collect()
}

// Пример 2 - преобразование String -> HashSet<char>
fn unique_letters(string: &str) -> HashSet<char> {
    string.chars().collect()
}

// Пример 3 - Vec<T> -> BTreeSet<T>
fn ordered_unique_values<T: Ord>(values: Vec<T>) -> BTreeSet<T> {
    values.into_iter().collect()
}
```
