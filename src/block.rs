pub struct Block {
  is_empty: bool,
  pos_x: i8,
  pos_y: i8
}

impl Block {
    pub fn new(is_empty: bool, pos_x: i8, pos_y: i8) -> Self { Self { is_empty, pos_x, pos_y } }

    pub fn get_is_empty(&self) -> bool { return self.is_empty }

    pub fn get_pos_x(&self) -> i8 { return self.pos_x }
    
    pub fn get_pos_y(&self) -> i8 { return self.pos_y }
}