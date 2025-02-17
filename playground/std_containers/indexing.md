# Индексирование коллекций

Официальная документация: [Index](https://doc.rust-lang.org/std/ops/trait.Index.html), [IndexMut](https://doc.rust-lang.org/std/ops/trait.IndexMut.html)

Оператор `[]` - индексирование коллекций, определяется трейтами `Index` и `IndexMut`

## Index, IndexMut

```rust
use std::ops::{Index, IndexMut};

trait Index<Idx: ?Sized> {
    type Output: ?Sized;

    fn index(&self, index: Idx) -> &Self::Output;
}

trait IndexMut<Idx>: Index<Idx> where Idx: ?Sized {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}


let vec = vec![1, 2, 3, 4, 5];
assert_eq!(&vec[..], &[1, 2, 3, 4, 5]);
assert_eq!(&vec[1..], &[2, 3, 4, 5]);
assert_eq!(&vec[..3], &[1, 2, 3]);
assert_eq!(&vec[1..3], &[2, 3]);

// Особенности 
// let num_ref: &i32 = *vec[0] - неявное разыменование при использовании оператора []
let num_ref: &i32 = vec[0]; // Expects &i32 found i32


let num_ref: i32 = vec[0];
let num_ref: &i32 = &vec[0];
```
