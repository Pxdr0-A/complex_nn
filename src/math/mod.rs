/* Math and Arithmetic Operations and Structures

- complex structures could be taken as pi fractions

 */

use std::f64::consts::PI;

// general complex numbers
#[derive(Debug, Clone)]
pub struct Cf64 {
    pub q: f64,
    pub p: f64
}

impl Cf64 {
    pub fn new(q: f64, p: f64) -> Cf64 {
        // if number is negative it is converted into positive
        Cf64 { q: q.abs(), p: p % PI }
    }

    pub fn add(z: &Cf64, w: &Cf64) -> Cf64 {
        let re = z.q * z.p.cos() + w.q * w.p.cos();
        let im = z.q * z.p.sin() + w.q * w.p.sin();

        Cf64 {
            q: complex_ops::norm_f64(&re, &im),
            p: complex_ops::phase_f64(&re, &im)
        }
    }

    pub fn mul(z: &Cf64, w: &Cf64) -> Cf64 {
        let q = z.q * w.q;
        let p = (z.p + w.p) % PI;

        Cf64 { q, p }
    }

    pub fn conj(&mut self) {
        self.p = -self.p;
    }
}


mod complex_ops {
    // consider macro for general operations
    pub fn norm_f64(re: &f64, im: &f64) -> f64 {
        re.powf(2.0) + im.powf(2.0)
    }

    pub fn phase_f64(re: &f64, im: &f64) -> f64 {
        (im.powf(2.0) / re.powf(2.0)).tan() % super::PI
    }
}

pub mod random {
    pub fn lcg(seed: u128) -> (u128, f64) {
        // IBM C/C++ convention params
        let a: u128 = 1103515245;
        let b: u128 = 12345;
        let m: u128 = 2u128.pow(31);

        let rand_num = (a*seed + b) %  (m - 1);
        let rand = (rand_num as f64)/(m as f64);

        (rand_num, rand)
    }
}