use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Chip8_cpu {
    opcode: u16,            // Operation calls
    memory: [u8; 4096],     // 4KB = 4096 bytes
    V: [u8; 16],            // Registers: V0 .. V16
    I: u16,                 // Index register
    pc: u16,                // Program counter
    gfx: [u8; 64*32],       // Graphics = 2048 pixels
    delay_timer: u8,        // 60Hz counter
    sound_timer: u8,        // 60HZ counter
    stack: [u16; 16],       // The call stack
    sp: u16,                // The stack pointer remembers the current location
    key: [u8; 16],          // Keyboard input
    fontset: [u8; 80]       // Default fontset
}

// initialize the Chip8 and set initial values to zero
impl Default for Chip8_cpu {
    fn default() -> Self {
        Self {
            opcode: 0,            
            memory: [0; 4096],     
            V: [0; 16],            
            I: 0,                 
            pc: 0,                
            gfx: [0; 64*32],       
            delay_timer: 0,        
            sound_timer: 0,        
            stack: [0; 16],       
            sp: 0,                
            key: [0; 16],
            fontset: [0; 80]
        }
    }
}
 
fn read_byte(c8: &mut Chip8_cpu, address: u16, value: u8) -> u8 {
   c8.memory[address as usize]
}

fn write_byte(c8: &mut Chip8_cpu, address: u16, value: u8) {
    c8.memory[address as usize] = value;
}

fn open_rom(c8: &mut Chip8_cpu, file_name: &str) {
    let mut file_buffer = File::open(file_name).unwrap();           // Open file
    let mut data = Vec::<u8>::new();                                // Use vector type array for the data
    file_buffer.read_to_end(&mut data);                             // Read the whole file and store data in 8 bits

    for i in 0..data.len() {
        write_byte(c8, (c8.pc + (i as u16)) as u16, data[i]);
    }

    //TODO: The rom should be read and placed into the memory 

    //TODO: Read file in binary mode because .rom files are binary
    //TODO: Read and write to memory functions for reading the rom file and converting to opcodes
    //TODO: Finish this function properly
}

// TODO: Add update function
// TODO: Add render (draw) function


fn init(c8: &mut Chip8_cpu) {
    c8.pc = 0x200;          // Program counter starts at memory location 512
    c8.opcode = 0x0;        // Set initial opcode to zero
    c8.I = 0x0;             // Set initial index to zero
    c8.sp = 0x0;            // Set the stack pointer to zero
    c8.fontset =
    [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
        0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
        0x90, 0x90, 0xf0, 0x10, 0x10, // 4
        0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
        0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
        0xf0, 0x10, 0x20, 0x40, 0x40, // 7
        0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
        0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
        0xf0, 0x90, 0xf0, 0x90, 0x90, // a
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F  
    ];
    
    for i in 0..80 {
        c8.memory[i] = c8.fontset[i]; // Load fontset into memory
    }
}

fn main() {
    let mut chip8 = Chip8_cpu::default();
    
    init(&mut chip8);
    open_rom(&mut chip8, "IBM Logo.ch8");
    
    println!("{chip8:#?}"); 
}
