#[allow(unused_imports)]
use rand::random;
use std::env;
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
    draw_flag: bool,    // Draw flag
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
            draw_flag: false,
        }
    }
}

fn emulate(c8: &mut Chip8_cpu) {
    c8.opcode = ((c8.memory[c8.pc as usize] as usize) << 8
        | (c8.memory[(c8.pc as usize) + (1 as usize)] as usize)) as u16; // Retrieve opcodes from file

    println!("Passed opcode: {}", c8.opcode);
    println!("BEGIN: Program counter is currently: {}", c8.pc); // Checking if the program counter is getting updated properly

    // TODO: Check opcodes
    match c8.opcode & 0xF000 {
        0x0000 => {
            match c8.opcode & 0x00FF {
                0x00E0 => {
                    // 0x00E0: Clears the screen
                    println!("Opcode: {} = 0x00E0", c8.opcode);

                    for i in 0..64 * 32 {
                        c8.gfx[i] = 0;
                    }

                    c8.draw_flag = true;
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
                _ => {
                    // println!("{c8:#?}");
                    panic!("Unknown or un-implemented opcode: {}", c8.opcode);
                }
            }
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
            // 0x3000: If VX == NN, skip next instruction
            println!("Opcode: {} = 0x3000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;

            if (c8.V[x as usize]) as usize == (c8.opcode & 0x00FF) as usize {
                c8.pc += 4;
            } else {
                c8.pc += 2;
            }
        }

        0x4000 => {
            // 0x4000: If VX != NN, skip next instruction
            println!("Opcode: {} = 0x4000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;

            if (c8.V[x as usize]) as usize != (c8.opcode & 0x00FF) as usize {
                c8.pc += 4;
            } else {
                c8.pc += 2;
            }
        }

        0x5000 => {
            // 0x5000: If VX == VY, skip next instruction
            println!("Opcode: {} = 0x5000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;
            let y = (c8.opcode & 0x00F0) >> 4;

            if (c8.V[x as usize]) as usize == (c8.V[y as usize]) as usize {
                c8.pc += 4;
            } else {
                c8.pc += 2;
            }
        }

        0x6000 => {
            // 0x6000: Sets VX to NN
            println!("Opcode: {} = 0x6000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;

            c8.V[x as usize] = (c8.opcode & 0x00FF) as u8;
            c8.pc += 2;
        }

        0x7000 => {
            // 0x7000: Add NN to VX
            println!("Opcode: {} = 0x7000", c8.opcode);
            println!("{c8:#?}"); // TODO: Fix error

            let x = (c8.opcode & 0x0F00) >> 8;

            c8.V[x as usize] += (c8.opcode & 0x00FF) as u8;
            c8.pc += 2;
        }

        0x9000 => {
            // 0x9000: If VX != VY, skip next instruction
            println!("Opcode: {} = 0x9000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;
            let y = (c8.opcode & 0x00F0) >> 4;

            if (c8.V[x as usize]) as usize != (c8.V[y as usize]) as usize {
                c8.pc += 4;
            } else {
                c8.pc += 2;
            }
        }

        0xD000 => {
            /*
            0xD000: Draws a sprite at coordinate (VX, VY) that has a width of 8
            pixels and a height of N pixels

            Each row of 8 pixels is read as bit-coded starting from memory
            location I;

            I value doesn't change after the execution of this instruction

            VF is set to 1 if any screen pixels are flipped from set to unset
            when the sprite is drawn, and to 0 if that doesn't happen
            */

            println!("Opcode: {} = 0xD000", c8.opcode);

            let x: u8 = c8.V[(c8.opcode as usize & 0x0F00) >> 8];
            let y: u8 = c8.V[(c8.opcode as usize & 0x00F0) >> 4];
            let height: u8 = (c8.opcode as usize & 0x000F) as u8;
            let mut pixel: u8;

            c8.V[0xF] = 0;

            for mut yline in 0..height {
                if yline <= height {
                    yline += 1;
                }

                pixel = c8.memory[c8.I as usize + yline as usize];

                for mut xline in 0..8 {
                    if xline < 7 {
                        xline += 1;
                    }

                    if pixel & (0x80 >> xline) != 0 {
                        if c8.gfx
                            [x as usize + xline as usize + ((y as usize + yline as usize) * 64)]
                            == 1
                        {
                            c8.V[0xF] = 1;
                        }
                        c8.gfx
                            [x as usize + xline as usize + ((y as usize + yline as usize) * 64)] ^=
                            1;
                    }
                }
            }

            c8.draw_flag = true;
            c8.pc += 2;
        }

        0xA000 => {
            //0xA000: Set I to the address NNN
            println!("Opcode: {} = 0xA000", c8.opcode);

            c8.I = c8.opcode & 0x0FFF;
            c8.pc += 2;
        }

        0xB000 => {
            //0xB000: Jump to address NN and set to V0
            println!("Opcode: {} = 0xC000", c8.opcode);

            c8.pc = ((c8.opcode & 0x0FFF) as usize + c8.V[0] as usize) as u16;
        }

        0xC000 => {
            // 0xC000: Sets VX to the result of a bitwise and operation on a random number and NN
            println!("Opcode: {} = 0xC000", c8.opcode);

            let x = (c8.opcode & 0x0F00) >> 8;

            c8.V[x as usize] =
                ((rand::random::<usize>() % 256) as usize & (c8.opcode & 0x00FF) as usize) as u8;
            c8.pc += 2;
        }

        // More opcodes
        _ => {
            // println!("{c8:#?}");
            panic!("Unknown or un-implemented opcode: {}", c8.opcode);
        }
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

    println!("Loaded ROM: {}", file_name);
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
    /*
     * IBM Logo.ch8: Working
     * test_opcode.ch8: Not working > err = add overflow
     * Rocket2.ch8: Not working > err = un-implemented opcode
     * INVADERS.ch8: Not working > err = un-implementd opcode
     * C8PIC.ch8: Not working > err = index out of bounds: len is 2049 but max is 2048 !!! 0xD000
     */

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Command line arguments cannot be zero.");
        return;
    } else if args[1] == "" || args[1] == " " {
        println!("Command line arguments cannot be zero.");
        return;
    } else {
        open_rom(&mut chip8, &args[1]);
    }

    // open_rom(&mut chip8, &args[1]);
    // println!("{chip8:#?}");

    loop {
        emulate(&mut chip8);
    }
}
