use super::*;
use eyre::Result;
use irrgarten::Maze;
use ndarray::Array2;

pub struct MazeGenerator;

impl MapGenerator for MazeGenerator {
    fn generate<R: Rng + ?Sized>(rng: &mut R, width: usize, height: usize) -> Result<TwMap> {
        // Must be odd.
        let width = {
            if width % 2 == 0 {
                width + 1
            } else {
                width
            }
        };
        let height = {
            if height % 2 == 0 {
                height + 1
            } else {
                height
            }
        };

        let mut map = create_initial_map()?;
        let maze = Maze::new(width, height).unwrap().generate(rng);

        let hookable_tiles =
            Array2::from_shape_fn((height, width), |(y, x)| {
                let mut t = 0;
                if maze[x][y] == 1 {
                    t = 9;
                }
                Tile::new(t, TileFlags::empty())
            });

        let mut tiles = Array2::from_shape_fn((width, height), |(y, x)| {
            GameTile::new(maze[x][y], TileFlags::empty())
        });

        // Put spawn and start tile on top left most tile.
        'outerStart: for y in 0..height {
            for x in 0..width {
                let tile = &mut tiles[(y, x)];
                if tile.id == 0 {
                    tile.id = TILE_SPAWN;
                    replace_around_gametile(&mut tiles, x, y, TILE_EMPTY, TILE_START);
                    break 'outerStart;
                }
            }
        }

        // Put finish tile on bottom right most tile.
        'outerFinish: for y in (0..height).rev() {
            for x in (0..width).rev() {
                let tile = &mut tiles[(y, x)];
                if tile.id == 0 {
                    *tile = GameTile::new(TILE_FINISH, TileFlags::empty());
                    break 'outerFinish;
                }
            }
        }

        let mut hook_tiles_layer = TilesLayer::new((height, width));
        hook_tiles_layer.image = Some(0);
        hook_tiles_layer.tiles = CompressedData::Loaded(hookable_tiles);

        let game_layer = GameLayer {
            tiles: CompressedData::Loaded(tiles),
        };

        let mut physics = Group::physics();
        physics.layers.push(Layer::Game(game_layer));
        physics.layers.push(Layer::Tiles(hook_tiles_layer));

        map.groups.push(quads_sky());
        map.groups.push(physics);

        Ok(map)
    }
}
