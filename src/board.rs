use rand::Rng;
use crate::block::Block;

pub struct Board {
  size_x: i8,
  size_y: i8,
  blocks: Vec<Vec<Block>>
}

impl Board {
  pub fn new(size_x: i8, size_y: i8) -> Self { Self { size_x, size_y, blocks: Vec::new() } }

  pub fn generate(&mut self) {
    for y in 0..self.size_y {
      let mut x_row: Vec<Block> = Vec::new();

      for x in 0..self.size_x {
        x_row.push(
          Block::new(
            rand::thread_rng().gen::<bool>(), 
            x, 
            y
        ));
      }

      self.blocks.push(x_row)
    }
  }

  pub fn build_blocks_map(self) -> Vec<Vec<bool>> {
    let mut block_map = Vec::new();

    for y_row in self.blocks {
      let mut row = Vec::new();
        
      for block in y_row {
        row.push(block.get_is_empty())
      } 

      block_map.push(row)
    }

    return block_map;
  }
}
