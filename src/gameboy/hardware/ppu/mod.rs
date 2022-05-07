use std::fmt;

use self::registers::Control;

pub mod registers;

#[derive(PartialEq, Eq)]
enum Mode {
    VBlank,
}

pub struct PPU {
    vram: Box<[u8]>,
    control: Control,
    current_line: u8,
    scroll_x: u8,
    scroll_y: u8,
}

impl PPU {
    pub fn new() -> Self {
        Self {
            vram: vec![0xFF; 8192].into_boxed_slice(),
            control: Control::empty(),

            current_line: 0,
            scroll_x: 0,
            scroll_y: 0,
        }
    }

    pub fn get_current_line(&self) -> u8 {
        self.current_line
    }

    pub fn get_control(&self) -> u8 {
        unimplemented!("get_control()")
    }

    pub fn set_control(&mut self, val: u8) {
        unimplemented!("set_control({:08b})", val)
    }

    pub fn get_scroll_y(&self) -> u8 {
        self.scroll_y
    }

    pub fn set_scroll_y(&mut self, val: u8) {
        self.scroll_y = val
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.vram[addr as usize] = val
    }
}

impl fmt::Debug for PPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Control:{:08b}", self.control.bits(), )
    }
}
