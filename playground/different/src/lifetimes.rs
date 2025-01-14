//! Времена жизни

struct SmartPlug {
    name: String,
}
struct SmartThermometer {
    name: String,
}

//
struct SmartHouseContainer<'a> {
    smart_device: &'a SmartPlug,
}

struct SmartServer<'b> {
    smart_house: SmartHouseContainer<'b>,
}

fn take_first(xs: &[i32]) -> &i32 {
    &xs[0]
}



// Доступны во время всей жизни программы
static STATIC_STR: &'static str = "Eternal";

fn main() {
    
    let int_in_heap = Box::new(10);
    let static_int: &'static i32 = Box::leak(int_in_heap);

    let my_plug = SmartPlug {
        // ---> Время жизни `x
        name: "Example".to_string(),
    };

    let my_house = SmartHouseContainer {
        // ---> SmartHouseContainer<'x>
        smart_device: &my_plug,
    };

    let my_server = SmartServer {
        // ---> SmartServer<'x>
        smart_house: my_house,
    };

    // 'my_server_ref: 'x (зависит от времени жизни x)
    // => 'my_server_ref должна быть не больше, чем x!!
    let my_server_ref = &my_server;

    //drop(my_plug); // => Error (Если в дальнейшем планируем использовать my_server_ref, my_server, my_house, my_plug)
    //println!("my_server_ref is {:?}", my_server_ref.smart_house.smart_device.name);

    let x = [1, 2, 3]; //`x
    let y = take_first(&x); // 'y: 'x

    println!("{y:?}");
}