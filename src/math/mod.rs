/* Math and Arithmetic Operations and Structures

- complex structures could be taken as pi fractions

 */

pub mod backpropagation;

use std::f64::consts::PI;

// general complex numbers
// consider enum with precision
#[derive(Debug, Clone)]
pub struct Cf64 {
    pub q: f64,
    pub p: f64
}

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    body: Vec<T>,
    shape: [usize; 2],
    capacity: [usize; 2]
}

impl Cf64 {
    pub fn new(q: f64, p: f64) -> Cf64 {
        // if number is negative it is converted into positive
        // phase is between -pi and pi
        Cf64 { q: q.abs(), p: p % (PI + 1e-5) }
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

impl<T> Matrix<T> {
    pub fn new(capacity: [usize; 2]) -> Matrix<T> {
        // allocates enough memory
        let body = Vec::with_capacity(capacity[0] * capacity[1]);
        let shape = [0, 0];

        Matrix { body, shape, capacity }
    }

    pub fn elm(&self, i: &usize, j: &usize) -> &T {
        // i - lines; j - columns
        // check for search validity
        let element = &self.body[i * self.shape[1] + j];

        element

    }

    pub fn add_row(&mut self, row: &mut Vec<T>) {
        // call inside an expression where mut Vec<f64> is declared
        // add verification for capacity and insertion

        self.shape[0] += 1;
        self.shape[1] = row.len();
        self.body.append(row);
    }
}

/*
pub struct TwoDimVec {
    body: Vec<f64>,
    shape: [usize; 2],
    capacity: [usize; 2]
}

impl TwoDimVec {
    pub fn new(capacity: [usize; 2]) -> TwoDimVec {
        // allocates enough memory
        let body = Vec::with_capacity(capacity[0]*capacity[1]);
        let shape = [0, 0];
        TwoDimVec { body, shape, capacity }
    }

    pub fn elm(&self, i: &usize, j: &usize) -> f64 {
        // i - lines; j - columns
        // check for search validity

        let elm = self.body[i*self.shape[1] + j];

        elm
    }

*/

mod complex_ops {
    // consider macro for general operations
    pub fn norm_f64(re: &f64, im: &f64) -> f64 {
        re.powi(2) + im.powi(2)
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