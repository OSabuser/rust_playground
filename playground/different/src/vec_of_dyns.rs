trait SmartDevice {
    fn get_status(&self) -> String;
}

struct Socket {
    name: String,
    power_consumption: f32,
}
impl SmartDevice for Socket {
    fn get_status(&self) -> String {
        format!(
            "Power consumption of socket {} is {} W",
            self.name, self.power_consumption
        )
    }
}

struct Thermometer {
    name: String,
    temperature: f32,
}
impl SmartDevice for Thermometer {
    fn get_status(&self) -> String {
        format!(
            "Current temperature of thermometer {} is {} W",
            self.name, self.temperature
        )
    }
}

struct SmartContainer<'a> {
    name: String,
    devices: Vec<Box<&'a dyn SmartDevice>>,
}
impl SmartContainer<'_> {
    fn show_devices(&self) {
        if !self.devices.is_empty() {
            for device in &self.devices {
                println!("{}", device.get_status());
            }
        } else {
            println!("There are no devices in {}!", self.name);
        }
    }
}

fn main() {
    let socket_1 = Socket {
        name: "Sample".to_string(),
        power_consumption: 0.1,
    };
    let thermometer_1 = Thermometer {
        name: "Tester".to_string(),
        temperature: 14.4,
    };

    let mut smart_house = SmartContainer {
        name: "SmartHome".to_string(),
        devices: Vec::new(),
    };

    smart_house.devices.push(Box::new(&socket_1));
    smart_house.devices.push(Box::new(&thermometer_1));

    smart_house.show_devices();
}


