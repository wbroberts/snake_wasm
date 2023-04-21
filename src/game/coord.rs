use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coord(pub isize, pub isize);

impl Coord {
    pub fn new_random(max_range: isize) -> Coord {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..max_range);
        let y = rng.gen_range(0..max_range);

        Coord(x, y)
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d, size: f64, color: &str) {
        let Self(x, y) = self;
        let x = *x as f64;
        let y = *y as f64;

        ctx.set_fill_style(&color.into());
        ctx.fill_rect(x * size, y * size, size - 1.0, size - 1.0);
    }

    pub fn clear(&self, ctx: &CanvasRenderingContext2d, size: f64) {
        let Self(x, y) = self;
        let x = *x as f64;
        let y = *y as f64;

        ctx.clear_rect(x * size, y * size, size - 1.0, size - 1.0);
    }
}
