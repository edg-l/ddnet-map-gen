use super::*;
use eyre::Result;
use ndarray::Array2;
use rand::Rng;

pub struct FlyGenerator;

impl MapGenerator for FlyGenerator {
    fn generate<R: Rng + ?Sized>(rng: &mut R) -> Result<TwMap> {
        let mut map = create_initial_map()?;

        const HEIGHT: usize = 1000;
        const WIDTH: usize = 100;

        let mut tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });
        let mut front_tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });

        let mut unhookable_tiles =
            Array2::from_shape_simple_fn((HEIGHT, WIDTH), || Tile::new(0, TileFlags::empty()));
        let mut freeze_tiles =
            Array2::from_shape_simple_fn((HEIGHT, WIDTH), || Tile::new(0, TileFlags::empty()));

        tiles
            .row_mut(HEIGHT - 2)
            .iter_mut()
            .for_each(|tile| tile.id = TILE_UNHOOKABLE);
        unhookable_tiles
            .row_mut(HEIGHT - 2)
            .iter_mut()
            .for_each(|tile| tile.id = 1);

        tiles[(HEIGHT - 3, WIDTH / 2)].id = TILE_SPAWN;

        for x in 0..WIDTH {
            front_tiles[(HEIGHT - 6, x)].id = TILE_START;
            front_tiles[(10, x)].id = TILE_FINISH;
        }

        let mut center: i64 = WIDTH as i64 / 2;
        let mut fly_width: i64 = 10;

        for y in (0..=(HEIGHT - 3)).rev() {
            let direction: i64 = rng.gen_range(-1..=1);
            let width_change: i64 = rng.gen_range(-1..=1);
            center += direction;
            fly_width += width_change;
            center = center.clamp(fly_width, WIDTH as i64 - fly_width);
            fly_width = fly_width.clamp(2, 12);

            for x in ((center + fly_width) as usize)..WIDTH {
                tiles[(y, x)].id = TILE_FREEZE;
                freeze_tiles[(y, x)].id = 4;
            }

            for x in 0..=((center - fly_width) as usize) {
                tiles[(y, x)].id = TILE_FREEZE;
                freeze_tiles[(y, x)].id = 4;
            }
        }

        let game_layer = GameLayer {
            tiles: CompressedData::Loaded(tiles),
        };

        let front_layer = FrontLayer {
            tiles: CompressedData::Loaded(front_tiles),
        };

        let mut unhook_tiles_layer = TilesLayer::new((HEIGHT, WIDTH));
        unhook_tiles_layer.image = Some(0);
        unhook_tiles_layer.tiles = CompressedData::Loaded(unhookable_tiles);

        let mut freeze_tiles_layer = TilesLayer::new((HEIGHT, WIDTH));
        freeze_tiles_layer.image = Some(1);
        freeze_tiles_layer.tiles = CompressedData::Loaded(freeze_tiles);
        freeze_tiles_layer.color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 200,
        };

        let mut physics = Group::physics();
        physics.layers.push(Layer::Game(game_layer));
        physics.layers.push(Layer::Front(front_layer));
        physics.layers.push(Layer::Tiles(unhook_tiles_layer));
        physics.layers.push(Layer::Tiles(freeze_tiles_layer));

        map.groups.push(quads_sky());
        map.groups.push(physics);

        Ok(map)
    }
}
