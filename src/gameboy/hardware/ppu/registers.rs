bitflags!(
  pub struct Control: u8 {
    const LCD_DISPLAY_ENABLE        = 0b_1000_0000;
    const WINDOW_TILE_MAP_ADDR      = 0b_0100_0000;
    const WINDOW_ENABLE             = 0b_0010_0000; 
    const BACKGROUND_TILE_DATA_ADDR = 0b_0001_0000; 
    const BACKGROUND_TILEM_MAP_ADDR = 0b_0000_1000; 
    const OBJ_SIZE                  = 0b_0000_0100; 
    const OBJ_ENABLE                = 0b_0000_0010; 
    const BACKGROUND_ENABLE         = 0b_0000_0001; 
  }
);

// bitflags!(
//   pub struct Stat: u8 {}
// );
