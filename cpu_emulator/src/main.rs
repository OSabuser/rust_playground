/// Описание процессора
/// *position_in_memory* - programm counter (регистр специального назначения)  
/// *registers* - регистры процессора  
/// *memory* - ОЗУ процессора = 4 кБ  
struct CPU {
    position_in_memory: usize,
    registers: [u8; 16],
    memory: [u8; 0x1000],
}
/// Функционал процессора
impl CPU {
    /* Чтение очередной кода операции из ОЗУ
     * current_operation - номер текущей операции
     */
    fn read_opcode(&self) -> u16 {
        let current_operation = self.position_in_memory;
        let op_byte_high = self.memory[current_operation] as u16;
        let op_byte_low = self.memory[current_operation + 1] as u16;

        op_byte_high << 8 | op_byte_low
    }

    fn run(&mut self) {
        loop {
            println!("CPU loop");
            println!("Current position on memory is: {}", self.position_in_memory);

            let opcode = self.read_opcode();

            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {return; },
                (0x08, _, _, 0x04) => self.add_xy(x, y),
                _ => todo!("opcode: {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg_1 = self.registers[x as usize];
        let arg_2 = self.registers[y as usize];

        /* Сложение с учётом возможного переполения
         * overflow = true, если зафиксировано переполнение
         */
        let (val, overflow) = arg_1.overflowing_add(arg_2);

        self.registers[x as usize] = val;

        // Установка флага переполнения
        self.registers[0x0F] = if overflow { 1 } else { 0 };
    }
}

fn main() {
    let mut cpu = CPU {
        position_in_memory: 0,
        memory: [0; 4096],
        registers: [0; 16],
    };

    // Инициализация значениями нескольких регистров
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 17;
    cpu.registers[3] = 13;

    // Загрузка 3 кодов операций для последовательного выполнения
    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14; // REG1 + REG0
    mem[2] = 0x80; mem[3] = 0x24; // REG2 + REG0
    mem[4] = 0x80; mem[5] = 0x34; // REG3 + REG0

    cpu.run();

    assert_eq!(cpu.registers[0], 45);

    println!("5 + 10 + 17+ 13 = {}", cpu.registers[0]);
}
