/* Math and Arithmetic Operations and Structures

- complex structures could be taken as pi fractions

 */

use std::f64::consts::PI;

// general complex numbers
// consider enum with precision
#[derive(Debug, Clone)]
pub struct Cf64 {
    pub q: f64,
    pub p: f64
}

impl Cf64 {
    pub fn new(q: f64, p: f64) -> Cf64 {
        // if number is negative it is converted into positive
        // phase is between -pi and pi
        Cf64 { q: q.abs(), p: p % (PI + 1e-5 ) }
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

    pub fn conj(&self) -> Cf64 {
        let q = self.q;
        let p = -self.p;

        Cf64 { q, p }
    }
}


mod complex_ops {
    // consider macro for general operations
    pub fn norm_f64(re: &f64, im: &f64) -> f64 {
        re.powf(2.0) + im.powf(2.0)
    }

    pub fn phase_f64(re: &f64, im: &f64) -> f64 {
        if re.is_sign_positive() && im.is_sign_positive() {
            // 1st quadrant (0 <-> pi/2)
            (im / re).atan()
        } else if re.is_sign_negative() && im.is_sign_positive() {
            // 2nd quadrant (pi/2 <-> pi)
            super::PI + (im / re).atan()
        } else if re.is_sign_negative() && im.is_sign_negative() {
            // 3rd quadrant (-pi <-> -pi/2)
            (im / re).atan() - super::PI
        } else {
            // 4th quadrant (-pi/2 <-> 0)
            (im / re).atan()
        }
    }
}

pub mod random {
    pub fn lcg(seed: &mut u128) -> f64 {
        // IBM C/C++ convention params
        let a: u128 = 1103515245;
        let b: u128 = 12345;
        let m: u128 = 2u128.pow(31);

        *seed = (a * *seed + b) % (m - 1);
        let rand = (*seed as f64) / (m as f64);

        rand
    }
}