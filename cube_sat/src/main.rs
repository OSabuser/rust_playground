//! Имитация работы системы спутниковой связи

/// Получение списка ID спутников
fn fetch_sat_ids() -> Vec<u32> {
    vec![1, 2, 3]
}
struct BaseStation;

impl BaseStation {
    fn connect(&self, sat_id: u32) -> CubeSat {
        CubeSat::new(sat_id)
    }
    /// Отправка сообщения message на спутник to
    fn send_message(&self, mailbox: &mut Mailbox, message: Message) {
        mailbox.post_message(message);
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
            mailbox: Mailbox {
                messages: Vec::new(),
            },
        }
    }

    /// Получение последнего сообщения из хранилища
    fn get_message(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.get_message(self)
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

impl Mailbox {
    fn post_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn get_message(&mut self, recipient: &CubeSat) -> Option<Message> {
        for message_number in 0..self.messages.len() {
            /// Сопоставление адресата имеющегося сообщения с recipient
            if self.messages[message_number].to == recipient.sat_id {
                let msg = self.messages.remove(message_number);
                return Some(msg);
            }
        }
        None
    }
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
    let mut mailbox = Mailbox { messages: vec![] };
    let main_station = BaseStation {};

    /// Получение ID доступных спутников
    let sat_identificators = fetch_sat_ids();

    /// Отправка сообщения
    for id in sat_identificators {
        /// Установка соединения с очередным спутником
        let sat_instance = main_station.connect(id);
        let current_id = sat_instance.sat_id;

        let msg = Message {
            to: id,
            content: String::from(format!("Hello, #{}", current_id)),
        };

        main_station.send_message(&mut mailbox, msg);
    }

    let sat_identificators = fetch_sat_ids();
    println!("Avaiable SATS: {:?}", sat_identificators);

    /// Чтение полученных сообщений
    for sat_id in sat_identificators {
        /// Установка соединения с очередным спутником
        let sat_instance = main_station.connect(sat_id);

        let received_message = match sat_instance.get_message(&mut mailbox) {
            Some(x) => x,
            None => Message {
                to: 0xFF,
                content: "------".to_string(),
            },
        };

        println!(
            "{:?} has received: {:?}",
            sat_instance, received_message
        );
    }
}
