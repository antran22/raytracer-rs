use crate::vec3::Point;

use super::rand_range;

type TrilinearData = [[[f64; 2]; 2]; 2];
const POINT_COUNT: usize = 256;
pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

pub trait NoiseFunction {
    fn noise(&self, point: &Point) -> f64;
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; POINT_COUNT];
        for item in randfloat.iter_mut().take(POINT_COUNT) {
            *item = rand_range(0.0..1.0);
        }

        Self {
            randfloat,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    fn generate_perm() -> [usize; POINT_COUNT] {
        let mut arr = [0_usize; POINT_COUNT];

        for (i, item) in arr.iter_mut().take(POINT_COUNT).enumerate() {
            *item = i;
        }

        for i in (1..POINT_COUNT).rev() {
            let j = rand_range(0..i);
            (arr[i], arr[j]) = (arr[j], arr[i]);
        }

        arr
    }

    fn terp(a: usize, b: f64) -> f64 {
        let a = a as f64;
        a * b + (1.0 - a) * (1.0 - b)
    }

    fn floor_and_fraction(a: f64) -> (isize, f64) {
        let floor = a.floor();
        let fraction = a - floor;

        (floor as isize, fraction)
    }

    fn hermitian_cubic(a: f64) -> f64 {
        a * a * (3.0 - 2.0 * a)
    }

    fn trilinear_interpolate(c: TrilinearData, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, c) in c.iter().enumerate() {
            for (j, c) in c.iter().enumerate() {
                for (k, c) in c.iter().enumerate() {
                    accum += Self::terp(i, u) * Self::terp(j, v) * Self::terp(k, w) * c
                }
            }
        }
        accum
    }
}

impl NoiseFunction for Perlin {
    fn noise(&self, p: &Point) -> f64 {
        let Perlin {
            perm_x,
            perm_y,
            perm_z,
            randfloat,
        } = self;

        let (i, u) = Self::floor_and_fraction(p.x);
        let (j, v) = Self::floor_and_fraction(p.y);
        let (k, w) = Self::floor_and_fraction(p.z);

        let [u, v, w] = [u, v, w].map(Self::hermitian_cubic);

        let mut c: TrilinearData = [[[0.0; 2]; 2]; 2];

        for (di, c) in c.iter_mut().enumerate() {
            for (dj, c) in c.iter_mut().enumerate() {
                for (dk, c) in c.iter_mut().enumerate() {
                    let di = ((i + di as isize) & 255) as usize;
                    let dj = ((j + dj as isize) & 255) as usize;
                    let dk = ((k + dk as isize) & 255) as usize;
                    *c = randfloat[perm_x[di] ^ perm_y[dj] ^ perm_z[dk]];
                }
            }
        }

        Self::trilinear_interpolate(c, u, v, w)
    }
}
