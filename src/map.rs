use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Tile {
    Sand,
    Wall,
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Map {
    id: String,
    tiles: Vec<Vec<Tile>>,
}

impl Default for Map {
    fn default() -> Self {
        let mut tiles = Vec::new();
        for y in 0..10 {
            let mut row = Vec::new();
            for x in 0..10 {
                row.push(if x == 0 || x == 9 || y == 0 || y == 9 { 
                    Tile::Wall
                } else {
                    Tile::Sand
                });
            }
            tiles.push(row);
        }
        Map {
            id: "0".to_owned(),
            tiles: tiles,
        }
    }
}