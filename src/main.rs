use serde_json;
use board::Board;

pub mod board;
pub mod block;

fn write_blocks_file(data: Vec<Vec<bool>>) {
    std::fs::write(
        "./boi.json",
        serde_json::to_string_pretty(&data).unwrap(),
    )
    .unwrap();
}

fn main() {
    let mut board = Board::new(5, 5);
    board.generate();
    let blocks_map = board.build_blocks_map();
    write_blocks_file(blocks_map)
}
