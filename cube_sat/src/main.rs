//! Имитация работы системы спутниковой связи

struct BaseStation;

#[derive(Debug)]
struct CubeSat {
    sat_id: u32,
}

impl CubeSat {
    fn new(id: u32) -> CubeSat {
        CubeSat {
            sat_id: id,
        }
    }
    
}

impl Copy for CubeSat {}

impl Clone for CubeSat {
    fn clone(&self) -> Self{
        CubeSat::new(self.sat_id)
    }
}


/// Вывод текущего состояния спутника
fn check_status(sat_id: CubeSat) {
    let status = StatusMessage::Ok;
    println!("Status of {:?} is {:?}", sat_id, status);
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}
impl Copy for StatusMessage {}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn main() {
    let sat_a = CubeSat::new(32);

    let a_status = check_status(sat_a);

    let a_status = check_status(sat_a);
}
