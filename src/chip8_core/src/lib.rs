const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [ 0xF0, 0x90, 0x90, 0x90, 0xF0, // 0 
                                      0x20, 0x60, 0x20, 0x20, 0x70, // 1 
                                      0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2 
                                      0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3 
                                      0x90, 0x90, 0xF0, 0x10, 0x10, // 4 
                                      0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5 
                                      0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6 
                                      0xF0, 0x10, 0x20, 0x40, 0x40, // 7 
                                      0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8 
                                      0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9 
                                      0xF0, 0x90, 0xF0, 0x90, 0x90, // A 
                                      0xE0, 0x90, 0xE0, 0x90, 0xE0, // B 
                                      0xF0, 0x80, 0x80, 0x80, 0xF0, // C 
                                      0xE0, 0x90, 0x90, 0x90, 0xE0, // D 
                                      0xF0, 0x80, 0xF0, 0x80, 0xF0, // E 
                                      0xF0, 0x80, 0xF0, 0x80, 0x80 // F 
                                    ];

pub const SCREEN_HEIGHT: usize = 32;
pub const SCREEN_WIDTH: usize = 64;

const RAM_SIZE: usize = 4096;
const NUMBER_OF_V_REGISTERS: usize = 16; 
const STACK_SIZE: usize = 16; 
const NUMBER_OF_KEYS: usize = 16; 

pub struct EmulatorSettings {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; NUMBER_OF_V_REGISTERS],
    i_reg: u16,
    stack: [u16; STACK_SIZE],
    keys: [bool; NUMBER_OF_KEYS],
    //stack pointer 
    sp: u16, 
    //delay timer
    dt: u8,
    //sound timer
    st: u8,
}


const START_ADDR: u16 = 0x200; //512 in decimal

impl EmulatorSettings {
    pub fn new() -> Self {
        Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUMBER_OF_V_REGISTERS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUMBER_OF_KEYS],
            dt: 0,
            st: 0,
        } 
    }

    pub fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    pub fn pop(&mut self) {
        self.sp -= 1;
        self.stack[self.sp as usize];
    }
}
