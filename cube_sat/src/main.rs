//! Имитация работы системы спутниковой связи

use std::fmt::format;
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

    fn connect(&self, sat_id: u32) -> CubeSat{
        CubeSat::new(sat_id)
    }
}


#[derive(Debug)]
struct CubeSat {
    sat_id: u32,
    mailbox: Mailbox,
}

fn fetch_sat_ids() -> Vec<u32> {
    vec![1,2]
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
#[derive(Debug)]
struct Message {
    to: u32,
    content: String,

}



fn main() {
    let main_station = BaseStation {};

    let sat_identificators = fetch_sat_ids();

    for id in sat_identificators {
        let mut sat_instance = main_station.connect(id);
        let id = sat_instance.sat_id;
        main_station.send_message(&mut sat_instance, format!("Hello, #{}", id));
    }
}
