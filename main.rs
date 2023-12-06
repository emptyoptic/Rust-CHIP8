use std::fs::File;
use std::io::Read;
// TODO: Once basic funtionality is implemented add SDL for drawing

#[derive(Debug)]
#[allow(dead_code)] // Will be removed once all is used
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
struct Chip8_cpu {
    opcode: u16,        // Operation calls
    memory: [u8; 4096], // 4KB = 4096 bytes
    V: [u8; 16],        // Registers: V0 .. V16
    I: u16,             // Index register
    pc: u16,            // Program counter
    gfx: [u8; 64 * 32], // Graphics = 2048 pixels
    delay_timer: u8,    // 60Hz counter
    sound_timer: u8,    // 60HZ counter
    stack: [u16; 16],   // The call stack
    sp: u16,            // The stack pointer remembers the current location
    key: [u8; 16],      // Keyboard input
    fontset: [u8; 80],  // Default fontset
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
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            fontset: [0; 80],
        }
    }
}

fn emulate(c8: &mut Chip8_cpu) {
    c8.opcode = ((c8.memory[c8.pc as usize] as usize) << 8
        | (c8.memory[(c8.pc as usize) + (1 as usize)] as usize)) as u16; // Retrieve opcodes from file

    println!("Passed first opcode: {}", c8.opcode);
    println!("Begin: Program counter is currently: {}", c8.pc); // Checking if the program counter is getting updated properly

    /*
    00E0 (clear screen)
    1NNN (jump)
    6XNN (set register VX)
    7XNN (add value to register VX)
    ANNN (set index register I)
    DXYN (display/draw)
    */

    match c8.opcode & 0xF000 {
        0x0000 => {
            match c8.opcode & 0x00FF {
                0x00E0 => {
                    // 0x00E0: Clears the screen
                    println!("Opcode: {} = 0x00E0", c8.opcode);

                    for i in 0..64 * 32 {
                        c8.gfx[i] = 0;
                    }

                    c8.pc += 2;
                }

                0x00EE => {
                    // 0x00EE: Returns from a subroutine
                    println!("Opcode: {} = 0x00EE", c8.opcode);

                    c8.pc = c8.stack[c8.sp as usize];
                    c8.sp = c8.sp - 1;
                    c8.pc += 2;
                }

                // More opcodes
                _ => panic!("Uknown opcode: {}", c8.opcode),
            }
        }

        0xA000 => {
            //0xA000: Set I to the address NNN
            println!("Opcode: {} = 0xA000", c8.opcode);

            c8.I = c8.opcode & 0x0FFF;
            c8.pc += 2;
        }

        0x1000 => {
            // 0x1000: Jumps to address NNN
            println!("Opcode: {} = 0x1000", c8.opcode);

            c8.pc = c8.opcode & 0x0FFF;
        }

        0x2000 => {
            // 0x2000: Call subroutine at NNN
            println!("Opcode: {} = 0x2000", c8.opcode);

            c8.sp += 1;
            c8.stack[c8.sp as usize] = c8.pc;
            c8.pc = c8.opcode & 0x0FFF;
        }

        0x3000 => {
            // 0x3000: If VX = NN, skip next instruction
            println!("Opcode: {} = 0x3000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;

            if (c8.V[x as usize]) as usize == (c8.opcode & 0x00FF) as usize {
                c8.pc += 2;
            }

            c8.pc += 2;
        }

        // More opcodes
        _ => panic!("Unknown opcode: {}", c8.opcode),
    }

    println!("END: Program counter is currently: {}", c8.pc); // Checking if the program counter is getting updated properly
}

#[allow(unused_must_use)]
fn open_rom(c8: &mut Chip8_cpu, file_name: &str) {
    let mut file_buffer = File::open(file_name).unwrap(); // Open file
    let mut data = Vec::<u8>::new(); // Use vector type array for the data
    file_buffer.read_to_end(&mut data); // Read the whole file and store data in 8 bits

    for i in 0..data.len() {
        c8.memory[(c8.pc as usize) + i] = data[i]; // Load file data into memory
    }
}

fn init(c8: &mut Chip8_cpu) {
    c8.pc = 0x200; // Program counter starts at memory location 512
    c8.opcode = 0x0; // Set initial opcode value to be zero
    c8.I = 0x0; // Set initial index value to be zero
    c8.sp = 0x0; // Set the stack pointer vlaue to be zero
    c8.fontset = [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    // TODO: Load keys into memory

    for i in 0..80 {
        c8.memory[i] = c8.fontset[i]; // Load fontset into memory
    }
}

// TODO ?: Add update function
// TODO: Add render (draw) function

fn main() {
    let mut chip8 = Chip8_cpu::default();

    init(&mut chip8);
    open_rom(&mut chip8, "INVADERS.ch8");
    //println!("{chip8:#?}");

    loop {
        emulate(&mut chip8);
    }
}
