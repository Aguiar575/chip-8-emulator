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
