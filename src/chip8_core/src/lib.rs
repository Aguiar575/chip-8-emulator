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
