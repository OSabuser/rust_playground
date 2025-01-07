trait MemoryWrite {
    fn write_to_memory(&mut self, buffer: &[u8]);
}

trait HasId {
    const ID: usize;
}

struct Hdd {
    memory_size: u32,
    memory_buffer: Vec<u8>,
    cache_buffer: Vec<u8>,
}

impl MemoryWrite for Hdd {
    fn write_to_memory(&mut self, buffer: &[u8]) {
        println!("Attempt for writting to HDD");
    }
}

struct Ram {
    memory_size: u32,
    memory_buffer: Vec<u8>,
    timings: (u8, u8, u8),
}

impl MemoryWrite for Ram {
    fn write_to_memory(&mut self, buffer: &[u8]) {
        println!("Attempt for writting to SDD");
    }
}
impl HasId for Ram {
    const ID: usize = 77;
}

fn main() {
    let mut hdd_1 = Hdd {
        memory_size: 1024,
        memory_buffer: vec![0; 1024],
        cache_buffer: vec![0; 256],
    };

    let mut ram_1 = Ram {
        memory_size: 128456,
        memory_buffer: vec![0; 128456],
        timings: (1, 2, 3),
    };

    let test_buffer = vec![0xFF, 0xAB, 0xAC];
    hdd_1.write_to_memory(&test_buffer);
    ram_1.write_to_memory(&test_buffer);

    let writer: &mut dyn MemoryWrite = &mut hdd_1;

    writer.write_to_memory(&test_buffer);

    println!("Ram ID is: {}", Ram::ID);
}
