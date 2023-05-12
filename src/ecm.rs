use num::complex::Complex;

pub fn ecm_impedance(freq: f64, ri: f64, rp: f64, c: f64) -> Complex<f64> {
    let omega = 2.0 * std::f64::consts::PI * freq;
    let z_rc = Complex::new(rp + ri, -omega * c).exp();
    let z_c = Complex::new(0.0, -1.0 / (omega * c));
    z_rc + z_c
}

