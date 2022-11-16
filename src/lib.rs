use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: i32,
    height: i32,
    draw_type: String,
    real: f64,
    imaginary: f64,
    cut_off: u32,
) -> Result<(), JsValue> {
    let mut gen = Generator::new(real, imaginary, (width, height), &draw_type, ctx, cut_off);
    gen.render()
}

struct Generator<'a> {
    c: Complex,
    data: Vec<u8>,
    ctx: &'a CanvasRenderingContext2d,
    cut_off: u32,
    dimensions: (i32, i32),
    draw_type: &'a str,
}
impl<'a> Generator<'a> {
    pub(crate) fn new(
        real: f64,
        imaginary: f64,
        dimensions: (i32, i32),
        draw_type: &'a str,
        ctx: &'a CanvasRenderingContext2d,
        cut_off: u32,
    ) -> Self {
        Self {
            c: Complex { real, imaginary },
            data: Vec::with_capacity((dimensions.0 * dimensions.1 * 4) as usize),
            ctx,
            dimensions,
            draw_type,
            cut_off,
        }
    }
    pub(crate) fn render(&mut self) -> Result<(), JsValue> {
        match self.draw_type {
            "julia" => {
                self.julia_set();
                self.set_canvas()
            }
            "mandel" => {
                self.mandel();
                self.set_canvas()
            }
            "ship" => {
                self.burning_ship();
                self.set_canvas()
            }
            _ => Ok(())
        }
    }
    pub(crate) fn set_canvas(&self) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&self.data),
            (self.dimensions.0 * 2) as u32,
            (self.dimensions.1 * 2) as u32,
        )?;
        self.ctx.put_image_data(&data, 0.0, 0.0)
    }

    fn julia_set(&mut self) {
        let param_i = 1.5;
        let param_r = 1.5;
        let scale = 0.005;

        for x in -self.dimensions.0..self.dimensions.0 {
            for y in -self.dimensions.1..self.dimensions.1 {
                let z = Complex {
                    real: y as f64 * scale - param_r,
                    imaginary: x as f64 * scale - param_i,
                };
                let b = Self::check_bound(z, self.c, self.cut_off);
                self.data.push((b / 4) as u8);
                self.data.push((b / 2) as u8);
                self.data.push(b as u8);
                self.data.push(255);
            }
        }
    }
    fn mandel(&mut self) {
        let cxmin = -2.0;
        let cxmax = 0.5;
        let cymin = -1.0;
        let cymax = 1.0;
        // let z = Complex {
        //     real: 0.0,
        //     imaginary: 0.0,
        // };
        for x in -self.dimensions.0..self.dimensions.0 {
            for y in -self.dimensions.1..self.dimensions.1 {
                let c = Complex {
                    real: (cxmin + x as f64) / (self.dimensions.0 as f64 - 1.0) * (cxmax - cxmin),
                    imaginary: (cymin + y as f64) / (self.dimensions.1 as f64 - 1.0) * (cymax - cymin),
                };
                let b = Self::check_bound(self.c, c, self.cut_off);
                if b == self.cut_off {
                    for _ in 0..3 {
                        self.data.push(0);
                    }
                    self.data.push(255);
                } else {
                    self.data.push((b % 8 * 32) as u8);
                    self.data.push(((b * 3) % 256) as u8);
                    self.data.push((b % 256) as u8);
                    self.data.push(255);
                }
            }
        }
    }
    fn burning_ship(&mut self) {
        let cxmin = -2.0;
        let cxmax = 0.5;
        let cymin = -1.0;
        let cymax = 1.0;

        for x in -self.dimensions.0..self.dimensions.0 {
            for y in -self.dimensions.1..self.dimensions.1 {
                let c = Complex {
                    real: (cxmin + x as f64) / (self.dimensions.0 as f64 - 1.0) * (cxmax- cxmin),
                    imaginary: (cymin + y as f64) / (self.dimensions.1 as f64 - 1.0) * (cymax - cymin),
                };
                let b = Self::check_bound_abs(self.c, c, self.cut_off);
                if b == self.cut_off {
                    for _ in 0..3 {
                        self.data.push(0);
                    }
                    self.data.push(255);
                } else {
                    self.data.push((b % 4 * 64) as u8);
                    self.data.push((b % 8 * 32) as u8);
                    self.data.push((b % 16 * 8) as u8);
                    self.data.push(255);
                }
            }
        }
    }
    fn bound_ratio(r: f64) -> f64 {
        let mut n = r;
        loop {
            let less = n < 0.0;
            let bigger = n > 1.0;
            if !less && !bigger {
                break n;
            }
            if less {
                n += 1.0;
            } else {
                n -= 1.0;
            }
        }
    }
    #[inline]
    fn calc_rgb_unit(u: f64, t1: f64, t2: f64) -> f64 {
        let mut r = t2;
        if 6.0 * u < 1.0 {
            r = t2 + (t1 - t2) * 6.0 * u;
        } else if 2.0 * u < 1.0 {
            r = t1
        } else if 3.0 * u < 2.0 {
            r = t2 + (t1 - t2) * (2.0 / 3.0 - u) * 6.0
        }
        r * 255.0
    }
    #[inline]
    fn hsv_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
        let s = s / 100.0;
        let v = v / 100.0;
        let c = s * v;
        let x = c * (1.0 - (((h / 60.0) % 2.0).abs() - 1.0));
        let m = v - c;
        let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);
        if (0.0..60.0).contains(&h) {
            r = c;
            g = x;
            b = 0.0;
        } else if (60.0..120.0).contains(&h) {
            r = x;
            g = c;
            b = 0.0;
        } else if (120.0..180.0).contains(&h) {
            r = 0.0;
            g = c;
            b = x;
        } else if (180.0..240.0).contains(&h) {
            r = 0.0;
            g = x;
            b = c;
        } else if (240.0..300.0).contains(&h) {
            r = x;
            g = 0.0;
            b = c;
        } else {
            r = c;
            g = 0.0;
            b = x;
        }
        let r = ((r + m) * 255.0) as u8;
        let g = ((g + m) * 255.0) as u8;
        let b = ((b + m) * 255.0) as u8;
        (r, g, b)
    }
    #[inline]
    fn hsl_to_rgb(h: f64, s: f64, v: f64) -> (u8, u8, u8) {
        let h = h / 360.0;
        let s = s / 360.0;
        let v = v / 360.0;

        if s == 0.0 {
            let u = 255.0 * v;
            return (u as u8, u as u8, u as u8);
        }
        let t = if v < 0.5 {
            v * (1.0 + s)
        } else {
            v + s - v * s
        };
        let t2 = 2.0 * v - t;
        let one_third = 1.0 / 3.0;
        let tr = Self::bound_ratio(h + one_third);
        let tg = Self::bound_ratio(h);
        let tb = Self::bound_ratio(h - one_third);

        let r = Self::calc_rgb_unit(tr, t, t2) * 255.0;
        let g = Self::calc_rgb_unit(tg, t, t2) * 255.0;
        let b = Self::calc_rgb_unit(tb, t, t2) * 255.0;
        (r as u8, g as u8, b as u8)
    }
    #[inline]
    fn check_bound(z: Complex, c: Complex, cut_off: u32) -> u32 {
        let mut bound: u32 = 0;
        let mut z = z;
        while bound < cut_off {
            if z.norm() > 2.0 {
                break;
            }
            z = z.square() + c;
            bound += 1;
        }
        bound
    }
    #[inline]
    fn check_bound_abs(z: Complex, c: Complex, cut_off: u32) -> u32 {
        let mut bound: u32 = 0;
        let mut z = z;
        while bound < cut_off {
            if z.norm().abs() > 2.0 {
                break;
            }
            z = z.abs().square() + c;
            bound += 1;
        }
        bound
    }
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    #[inline]
    fn square(self) -> Complex {
        let real = (self.real * self.real) - (self.imaginary * self.imaginary);
        let imaginary = 2.0 * self.real * self.imaginary;
        Complex { real, imaginary }
    }

    #[inline]
    fn norm(&self) -> f64 {
        (self.real * self.real) + (self.imaginary * self.imaginary)
    }

    #[inline]
    fn abs(&self) -> Self {
        Self {
            real: self.real.abs(),
            imaginary: self.imaginary.abs(),
        }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, rhs: Complex) -> Complex {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}
