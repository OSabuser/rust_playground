/// Описание процессора
/// *position_in_memory* - programm counter (регистр специального назначения)  
/// *registers* - регистры процессора  
/// *memory* - ОЗУ процессора = 4 кБ  
struct CPU {
    position_in_memory: usize,
    registers: [u8; 16],
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
    loop_counter: u32,
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
            let ret_val = self.loop_counter.overflowing_add(1);

            if ret_val.1 == true {
                panic!("kernel panic: loop_counter overflow!");
            }

            self.loop_counter = ret_val.0;

            println!("CPU loop: #{}", self.loop_counter);
            println!("Current position on memory is: 0x{:X}\n", self.position_in_memory);

            let opcode = self.read_opcode();

            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return;},
                (0x00, 0x00, 0x0E, 0x0E) => self.ret(),
                (0x02, _, _, _) => self.call(nnn),
                (0x08, _, _, 0x04) => self.add_xy(x, y),
                _ => todo!("opcode: {:04x}", opcode),
            }
        }
    }

    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        // Переход на addr
        self.position_in_memory = addr as usize;

        println!("Jump to: 0x{:X}", addr);
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        // Переход на адрес, откуда был сделан вызов
        self.position_in_memory = call_addr as usize;

        println!("Return to: 0x{:X}", call_addr);
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

    fn load_function_to_ram(&mut self, function: &[u8], addr: u16) {
        
        let function_len = function.len();
        let function_location = addr as usize;
        
        assert_ne!(function_len, 0, "Incorrect function format!");
        assert!(function_len % 2 == 0, "Incorrect function format!");
        assert!(function_location < self.memory.len(), "Bus fault!");
        
        let mem = &mut self.memory;

        mem[function_location..(function_location + function_len)].copy_from_slice(function);
    }


}

fn main() {
    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];

    let start_sequence = [0x21, 0x00, 0x21, 0x00, 0x00, 0x00];

    let mut cpu = CPU {
        position_in_memory: 0,
        memory: [0; 4096],
        registers: [0; 16],
        stack: [0; 16],
        stack_pointer: 0,
        loop_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 15;

    cpu.load_function_to_ram(&start_sequence, 0);

    cpu.load_function_to_ram(&add_twice, 0x100);

    cpu.run();

    println!("Result of operation: {}", cpu.registers[0]);

    
}
