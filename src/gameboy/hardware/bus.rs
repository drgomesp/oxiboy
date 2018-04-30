pub trait Bus {
    fn read(&self, addr: u16) -> u8;
}
