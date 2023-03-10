use super::*;
use eyre::Result;
use ndarray::Array2;
use rand::Rng;
use rand_distr::{Distribution, Uniform};

pub struct FlyGenerator {
    pub max_fly_width: u16,
    pub min_fly_width: u16,
}

impl MapGenerator for FlyGenerator {
    fn generate<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        mapres: &Path,
        width: usize,
        height: usize,
    ) -> Result<TwMap> {
        let mut map = create_initial_map(mapres)?;

        let mut game_tiles = Array2::from_shape_simple_fn((height, width), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });
        let mut front_tiles = Array2::from_shape_simple_fn((height, width), || {
            GameTile::new(TILE_EMPTY, TileFlags::empty())
        });

        let mut unhookable_tiles =
            Array2::from_shape_simple_fn((height, width), || Tile::new(0, TileFlags::empty()));
        let mut freeze_tiles =
            Array2::from_shape_simple_fn((height, width), || Tile::new(0, TileFlags::empty()));

        // Create the ceiling and spawn.
        game_tiles
            .row_mut(height - 2)
            .iter_mut()
            .for_each(|tile| tile.id = TILE_UNHOOKABLE);
        unhookable_tiles
            .row_mut(height - 2)
            .iter_mut()
            .for_each(|tile| tile.id = 2);

        game_tiles[(height - 3, width / 2)].id = TILE_SPAWN;

        // Place the start and finish tiles.
        for x in 0..width {
            front_tiles[(height - 6, x)].id = TILE_START;
            front_tiles[(10, x)].id = TILE_FINISH;
        }

        let mut center: i64 = width as i64 / 2;
        let mut fly_width: i64 = 10;

        let max_steps = (height as f64 * 0.30f64).round() as i64;

        let mut direction_steps: i64 = rng.gen_range((max_steps / 2)..=max_steps);
        let mut direction: i64 = rng.gen_range(-1..=1);

        let direction_steps_sampler = Uniform::from(1..=10);
        let direction_sampler = Uniform::from(-1..=1);
        let width_sampler = Uniform::from(-1..=1);

        for y in (0..=(height - 3)).rev() {
            if direction_steps == 0 {
                direction_steps = direction_steps_sampler.sample(rng);
                direction = direction_sampler.sample(rng);
            }

            let width_change: i64 = width_sampler.sample(rng);
            center += direction;
            fly_width += width_change;
            fly_width = fly_width.clamp(self.min_fly_width as i64, self.max_fly_width as i64);
            center = center.clamp(fly_width, width as i64 - fly_width - 1);

            for x in ((center + fly_width) as usize)..width {
                game_tiles[(y, x)].id = TILE_FREEZE;
                freeze_tiles[(y, x)].id = 4;
            }

            for x in 0..=((center - fly_width) as usize) {
                game_tiles[(y, x)].id = TILE_FREEZE;
                freeze_tiles[(y, x)].id = 4;
            }

            direction_steps -= 1;
        }

        let game_layer = GameLayer {
            tiles: CompressedData::Loaded(game_tiles),
        };

        let front_layer = FrontLayer {
            tiles: CompressedData::Loaded(front_tiles),
        };

        let mut unhook_tiles_layer = TilesLayer::new((height, width));
        unhook_tiles_layer.image = Some(0);
        unhook_tiles_layer.tiles = CompressedData::Loaded(unhookable_tiles);

        let mut freeze_tiles_layer = TilesLayer::new((height, width));
        freeze_tiles_layer.image = Some(1);
        freeze_tiles_layer.tiles = CompressedData::Loaded(freeze_tiles);
        freeze_tiles_layer.color = Rgba::new(0, 0, 0, 200);

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
