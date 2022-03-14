use irrgarten::Maze;
use ndarray::Array2;
use rand::Rng;
use rand_distr::{Normal, Distribution};
use twmap::*;
use color_eyre::Result;

const TILE_EMPTY: u8 = 0;
const TILE_HOOKABLE: u8 = 1;
const TILE_UNHOOKABLE: u8 = 3;
const TILE_FREEZE: u8 = 9;
const TILE_UNFREEZE: u8 = 1;
const TILE_START: u8 = 33;
const TILE_FINISH: u8 = 34;
const TILE_SPAWN: u8 = 192;

fn main() -> Result<()> {
    color_eyre::install()?;

    gen_flymap()?;

    Ok(())
}

fn create_initial_map() -> Result<TwMap> {
    let mut map = TwMap::empty(Version::DDNet06);
    map.info.author = "Ryozuki's Map Gen".to_string();
    map.info.credits = "Ryozuki's Map Gen".to_string();
    map.images.push(Image::External(ExternalImage {
        name: "generic_unhookable".to_string(),
        width: 1024,
        height: 1024,
    }));
    map.images.push(Image::Embedded(EmbeddedImage::from_file("mapres/basic_freeze.png")?));
    let physics = Group::physics();
    map.groups.push(physics);
    Ok(map)
}

fn gen_flymap() -> Result<()> {
    let mut rng = rand::thread_rng();

    let mut map = create_initial_map()?;

    const HEIGHT: usize = 1000;
    const WIDTH: usize = 100;

    let mut tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || GameTile::new(TILE_EMPTY, TileFlags::empty()));
    let mut front_tiles = Array2::from_shape_simple_fn((HEIGHT, WIDTH), || GameTile::new(TILE_EMPTY, TileFlags::empty()));

    tiles.row_mut(HEIGHT - 2).iter_mut().for_each(|tile| tile.id = TILE_UNHOOKABLE);
    tiles[(HEIGHT - 3, WIDTH / 2)].id = TILE_SPAWN;

    for x in 0..WIDTH {
        front_tiles[(HEIGHT - 6, x)].id = TILE_START;
        front_tiles[(10, x)].id = TILE_FINISH;
    }

    let mut center: i64 = WIDTH as i64 / 2;
    let mut fly_width: i64 = 10;

    for y in (0..=(HEIGHT-3)).rev() {
        let direction: i64 = rng.gen_range(-1..=1);
        let width_change: i64 = rng.gen_range(-1..=1);
        center += direction;
        fly_width += width_change;
        center = center.clamp(fly_width, WIDTH as i64 - fly_width);
        fly_width = fly_width.clamp(2, 12);

        for x in ((center + fly_width) as usize)..WIDTH {
            tiles[(y, x)].id = TILE_FREEZE;
        }

        for x in 0..=((center - fly_width) as usize) {
            tiles[(y, x)].id = TILE_FREEZE;
        }
    }

    let game_layer = GameLayer {
        tiles: CompressedData::Loaded(tiles)
    };

    let front_layer = FrontLayer {
        tiles: CompressedData::Loaded(front_tiles),
    };

    let physics = map.physics_group_mut();
    physics.layers.push(Layer::Game(game_layer));
    physics.layers.push(Layer::Front(front_layer));

    map.save_file("server/maps/generated.map")?;

    Ok(())
}

fn gen_maze() -> Result<()> {
    let mut rng = rand::thread_rng();

    let mut map = create_initial_map()?;

    let maze = Maze::new(1001, 1001).unwrap().generate(&mut rng);

    let mut tiles = Array2::from_shape_fn((1001, 1001), |(x, y)| GameTile::new(maze[x][y], TileFlags::empty()));

    // Put spawn and start tile on top left most tile.
    let mut added_spawn = false;
    'outerStart: for y in 0..101 {
        for x in 0..101 {
            let tile = &mut tiles[(x, y)];
            if tile.id == 0 && !added_spawn {
                *tile = GameTile::new(TILE_SPAWN, TileFlags::empty());
                added_spawn = true;
            }
            else if tile.id == 0 && added_spawn {
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
        tiles: CompressedData::Loaded(tiles)
    };

    let physics = map.physics_group_mut();
    physics.layers.push(Layer::Game(game_layer));

    map.save_file("server/maps/generated.map")?;

    Ok(())
}