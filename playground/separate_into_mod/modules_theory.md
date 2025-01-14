# Разбиение кода на модули  

- Модули - это иерархические области видимости  
- Модули могут быть описаны в одном `.rs` файле, а также однозначно распределены по нескольким **(имя файла = имя модуля)**


## Пример структуры крейта  

```rust
// Корневой модуль (lib.rs)
mod crate {
    // Подмодуль "Сервер"
    mod server {
        mod database {
            struct Db {}
        }

        mod protocol {
            enum Message {}
        }

        struct Server {}
    }

    // Подмодуль "Клиент"
    mod client {
        struct Descriptor {}
    }
}
```

## Обращение к сущностям крейта  

1. Абсолютные пути
2. Относительные пути  

> `crate` - корень библиотки - единица компиляции
```rust
// Абсолютный путь до Db
crate::server::database::Db

// Имя проекта (в cargo.toml) можно использовать для // доступа к сущностям крейта
my_project::client::Descriptor

// Относительные пути
// crate / my_project / super
// Находимся внутри модуля server:
self::database::Db
self::Server
super::client::Descriptor
```

## Модули и файлы
```
src/
    lib.rs / main.rs
    server
        database.rs
        protocol.rs
        mod.rs
    client.rs
```
```rust
//lib.rs
mod server;
mod client;

//server/mod.rs
mod database;
mod protocol;

//client.rs
struct Client;
```

### Оператор use

> Хороший тон - использовать use рядом с тем местом, где будет использоваться импортированная сущность

1. `use crate::path::to::module;`
2. `use module::Struct;`
3. `use module::{Struct, func, Trait};`
4. `use module::{func, submodule::another_func};`
5. `use module::*;`
6. `use module::item as SomeAlias;`
7. `use module::Trait as _;`


### Приватность  

> По умолчанию все сущности в Rust являются приватными
Приватные сущности можно использовать либо в самом модуле, либо в дочерних модулях.  

> Добавление ключевого слова `pub` позволяет импортировать сущности в другие модули  

> Когда *trait, enum* является публичным, всё его содержимое становится также публичным

> В **структурах** поля могут быть как приватными, так и публичными по отдельности

```rust
pub struct View {
    parent: Node,
    pub camera: Camera2d,
    children: Vec<Child>,
}

impl View {
    pub fn new(parent: &Node) -> Self {}
}

// Можем:
let view = View::new(&parent);
let camera_instance = &view.camera;
// Не можем:
let view = View {...};
let parent = view.parent;
```


### pub use

> Можем использовать публичную сущность из приватного модуля  

```rust
pub mod useful_stuff {
    // Реэкспорт
    pub use internal_stuff::Thing;

    // Приватный модуль
    mod internal_stuff {
        pub struct Thing;
    }
}

```

### pub  

- `Нет pub` - приватно (только текущий и дочерний модули)  
- `pub` - публично (+ родительский модуль, если текущий модуль pub)  
- `pub(crate)` - публично в пределах крейта  
- `pub(super)` - публично в пределах родительского модуля
- `pub(in some::path)` - публично в пределах указанного модуля
- `pub(self)` - аналогично отсутствию pub  

