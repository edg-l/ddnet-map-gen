use super::*;
use eyre::Result;
use irrgarten::Maze;
use ndarray::Array2;

pub struct MazeGenerator;

impl MapGenerator for MazeGenerator {
    fn generate<R: Rng + ?Sized>(rng: &mut R) -> Result<TwMap> {
        let mut map = create_initial_map()?;
        let maze = Maze::new(1001, 1001).unwrap().generate(rng);

        let mut tiles = Array2::from_shape_fn((1001, 1001), |(x, y)| {
            GameTile::new(maze[x][y], TileFlags::empty())
        });

        // Put spawn and start tile on top left most tile.
        let mut added_spawn = false;
        'outerStart: for y in 0..101 {
            for x in 0..101 {
                let tile = &mut tiles[(x, y)];
                if tile.id == 0 && !added_spawn {
                    *tile = GameTile::new(TILE_SPAWN, TileFlags::empty());
                    added_spawn = true;
                } else if tile.id == 0 && added_spawn {
                    *tile = GameTile::new(TILE_START, TileFlags::empty());
                    break 'outerStart;
                }
            }
        }

        // Put finish tile on bottom right most tile.
        'outerFinish: for y in (0..1001).rev() {
            for x in (0..1001).rev() {
                let tile = &mut tiles[(x, y)];
                if tile.id == 0 {
                    *tile = GameTile::new(TILE_FINISH, TileFlags::empty());
                    break 'outerFinish;
                }
            }
        }

        let game_layer = GameLayer {
            tiles: CompressedData::Loaded(tiles),
        };

        let mut physics = Group::physics();
        physics.layers.push(Layer::Game(game_layer));
        map.groups.push(physics);

        Ok(map)
    }
}
