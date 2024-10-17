# Проблема
> 5 философов, 5 вилок, одна общая тарелка со спагетти. Для того, чтобы поесть, каждому философу нужно 2 вилки сразу.

Простой пример решения этой проблемы:
1. Философ берет вилку в свою левую руку.
2. Затем берет вилку в свою правую руку.
3. Ест.
4. Кладет вилки на место.

Последовательность действий философов:
1. Философ 1 начинает выполнять алгоритм, берет вилку в левую руку.
2. Философ 2 начинает выполнять алгоритм, берет вилку в левую руку.
3. Философ 3 начинает выполнять алгоритм, берет вилку в левую руку.
4. Философ 4 начинает выполнять алгоритм, берет вилку в левую руку.
5. Философ 5 начинает выполнять алгоритм, берет вилку в левую руку.
6. ...? Все вилки заняты и никто не может начать есть! Безвыходное состояние


Объявление "статической функции" new для типа *Philosopher*
`
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        // Возвращаем экземпляр структуры Philosopher
        Philosopher {
            name: name.to_string(),
        }
    }
}`

`.to_string()` создаёт копию строки, на которую указывает *&str* и возвращает новый экземпляр *String*,
котороый будет присвоен полю name структуры Philosopher.

`new()` 
> Соглашение об именовании функций, которые возвращают новые экземпляры структур
> let p1 = Philosopher::new("Джудит Батлер") <=> let p1 = Philosopher {name: "Джудит Батлер".to_string()};
`Vec<T>`
> Вектор - расширяемая версия массива.
`for philosopher in &philosophers`
> Перебор вектора, получение ссылки на очередного философа на каждой итерации

# Методы & статические функции
В RUST методы явно получают параметр `self`. eat() - метод, new() - статическая функция.

`thread::sleep(time::Duration::from_secs(1))`
Останавливает рабочий поток на 1 секунду