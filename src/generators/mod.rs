use std::path::Path;

use color_eyre::Help;
use eyre::Result;
use fixed::types::I17F15;
use ndarray::Array2;
use rand::Rng;
use twmap::*;
use vek::{Extent2, Rgba};

pub mod fly;
pub mod maze;

pub const TILE_EMPTY: u8 = 0;
pub const TILE_HOOKABLE: u8 = 1;
pub const TILE_UNHOOKABLE: u8 = 3;
pub const TILE_FREEZE: u8 = 9;
pub const TILE_UNFREEZE: u8 = 1;
pub const TILE_START: u8 = 33;
pub const TILE_FINISH: u8 = 34;
pub const TILE_SPAWN: u8 = 192;

pub trait MapGenerator {
    fn generate<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        mapres: &Path,
        width: usize,
        height: usize,
    ) -> Result<TwMap>;

    fn save_file<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        mapres: &Path,
        width: usize,
        height: usize,
        path: &Path,
    ) -> Result<()> {
        let mut map = self.generate(rng, mapres, width, height)?;
        map.save_file(path).note("Failed to save the map file.")?;
        Ok(())
    }
}

pub fn create_initial_map(mapres: &Path) -> Result<TwMap> {
    let mut map = TwMap::empty(Version::DDNet06);
    map.info.author = "github.com/edg-l/ddnet-map-gen".to_string();
    map.info.credits = "github.com/edg-l/ddnet-map-gen".to_string();
    //map.info.version =
    map.images.push(Image::External(ExternalImage {
        name: "generic_unhookable".to_string(),
        size: Extent2::new(1024, 1024),
    }));
    map.images.push(Image::Embedded(EmbeddedImage::from_file(mapres.join("basic_freeze.png")).note("Might have failed due to a non existing mapres directory, check out the '--mapres' option.")?));
    Ok(map)
}

// Creates the sky quad from the editor.
pub fn quads_sky() -> Group {
    let mut quads_group = Group::default();
    let mut quads_layer = QuadsLayer::default();
    quads_group.parallax.x = 0;
    quads_group.parallax.y = 0;

    let mut quad = Quad::new(
        Default::default(),
        Extent2::new(I17F15::from_num(50), I17F15::from_num(30)),
    )
    .unwrap();
    quad.colors = [
        Rgba::new(94, 132, 174, 255),
        Rgba::new(94, 132, 174, 255),
        Rgba::new(204, 232, 255, 255),
        Rgba::new(204, 232, 255, 255),
    ];

    quads_layer.quads.push(quad);

    quads_group.layers.push(Layer::Quads(quads_layer));
    quads_group
}

// Changed the id of the tile if matches oldid.
pub fn replace_gametile(tiles: &mut Array2<GameTile>, x: usize, y: usize, oldid: u8, newid: u8) {
    if tiles[(y, x)].id == oldid {
        tiles[(y, x)].id = newid;
    }
}

pub fn replace_around_gametile(
    tiles: &mut Array2<GameTile>,
    x: usize,
    y: usize,
    oldid: u8,
    newid: u8,
) {
    let width = tiles.ncols();
    let height = tiles.nrows();

    let directions = [-1, 0, 1];

    for diry in directions {
        for dirx in directions {
            if dirx == 0 && diry == 0 {
                continue;
            }
            if (y as i64) + diry < 0 || (y as i64) + diry >= height as i64 {
                continue;
            }
            if (x as i64) + dirx < 0 || (x as i64) + dirx >= width as i64 {
                continue;
            }
            replace_gametile(
                tiles,
                ((x as i64) + dirx) as usize,
                ((y as i64) + diry) as usize,
                oldid,
                newid,
            );
        }
    }
}
