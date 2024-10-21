//! Имитация работы системы спутниковой связи
struct BaseStation;

impl BaseStation {
    /// Отправка сообщения message на спутник to
    fn send_message(
        &self,
        to: &mut CubeSat,
        message: Message
    ) {
        to.mailbox.messages.push(message);
    }
}


#[derive(Debug)]
struct CubeSat {
    sat_id: u32,
    mailbox: Mailbox,
}

impl CubeSat {
    
    fn new(id: u32) -> CubeSat {
        CubeSat {
            sat_id: id,
            mailbox: Mailbox {messages: Vec::new()}
        }
    }

    /// Получение последнего сообщения из хранилища
    fn receive_message(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
    /// Вывод текущего состояния спутника
    fn check_status(&self) {
        let status = StatusMessage::Ok;
        println!("Status of {:?} is {:?}", self, status);
    }
    
}


#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}


#[derive(Debug)]
enum StatusMessage {
    Ok,
}

type Message=String;



fn main() {
    let main_station = BaseStation {};

    let mut sat_a = CubeSat::new(1);
    let mut sat_b = CubeSat::new(2);

    main_station.send_message(&mut sat_a, "Hellow, sat_a!".to_string());
    main_station.send_message(&mut sat_b, String::from("Баояо-баояо!"));

    let current_message = sat_a.receive_message();
    println!("Sat #{} has received: {:?}", sat_a.sat_id, current_message);
    sat_a.check_status();

    let current_message = sat_b.receive_message();
    println!("Sat #{} has received: {:?}", sat_b.sat_id, current_message);
    sat_b.check_status();
}
