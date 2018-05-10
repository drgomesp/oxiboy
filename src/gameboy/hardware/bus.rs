use failure::Error;

pub trait MemoryBus {
    fn read(&self, addr: u16) -> Result<u8, Error>;
    fn write(&mut self, addr: u16, val: u8) -> Result<(), Error>;
}
