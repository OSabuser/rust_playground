# Замыкания 

## Синтаксис  

```rust
// || {}
|x| {println!("{x}"); x};
|x: i32| -> i32 {println!("{x}"); x}; // с типами возвращаемого значения и аргумента
|x, y| x - y // несколько аргументов
```  

***Замыкания захватывают своё окружение***
> Локальные переменные, находящиеся в той же области видимсоти, что и замыкание
> хранятся внутри замыкания!


### Захват локальной переменной factor (может быть мутабельной)  

```rust 
let factor = 3;  
let mul_by = |x| x * factor;  
let x = 5;  
let y = mul_by(x); # 15 
```

### Захват окружения мутабельно  

```rust 
fn requires_mut(v: &mut i32) {}
let mut x = 10;
let closure = || requires_mut(&mut x);
let r = &x; // ошибка создание новой ссылки при наличие одной мутабельной
closure();
```

### Захват окружения по значению  

> Перемещение локальной переменной внутрь замыкания с захватом владения
```rust
let x = vec![1, 2, 3];
let closure = move |v| v == x;
println!("{x:?}"); // Ошибка:

```

### Возврат замыканий  

```rust
fn returns_closure<F> () -> Box<dyn Fn(i32) -> i32> {
 Box::new(if cond { |x| x} else { |x| x * 3})
}
```
