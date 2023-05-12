mod ecm;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


fn main() -> Result<(), Box<dyn Error>> {
    let ri = 0.1;
    let rp = 0.2;
    let c = 1e-6;  // 1 uF
    
    let start_freq = 1000.0;
    let end_freq = 0.01;
    let num_points = 30;
    let freqs = logspace(start_freq, end_freq, num_points);

    // Accumulate impedance data
    let mut freq_data = Vec::with_capacity(num_points);
    let mut real_data = Vec::with_capacity(num_points);
    let mut imag_data = Vec::with_capacity(num_points);

    for freq in freqs {
        let z = ecm::ecm_impedance(freq, ri, rp, c);
        freq_data.push(freq);
        real_data.push(z.re);
        imag_data.push(z.im);

        println!("ECM impedance at {:.2} Hz: {:.4} + j{:.4} ohms",
                 freq, z.re, z.im);
    }

    // Save impedance data to CSV file
    let mut file = File::create("ecm_impedance.csv")?;
    file.write_all(b"f,Z_re,Z_im\n")?;
    for i in 0..num_points {
        file.write_fmt(format_args!("{:.2},{:.4},{:.4}\n",
            freq_data[i], real_data[i], imag_data[i]))?;
    }

    Ok(())
}

fn logspace(start: f64, end: f64, num_points: usize) -> Vec<f64> {
    let log_start = start.log10();
    let log_end = end.log10();
    let step = (log_end - log_start) / (num_points - 1) as f64;
    let mut freqs = Vec::with_capacity(num_points);
    for i in 0..num_points {
        let log_freq = log_start + i as f64 * step;
        freqs.push(10.0f64.powf(log_freq));
    }
    freqs
}
