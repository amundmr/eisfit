
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn read_data(filename: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    //Open file, read it and spit out 3 vectors of floats: Frequency, and impedance real and imaginary parts.

    
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines.skip(1) {
            if let Ok(ip) = line {
                let mut split = ip.split(",");
                let vec: Vec<&str> = split.collect();
                freq.push(vec[0].parse::<f64>().unwrap());
                println!("{}", freq[0]);
            }
        }
    }

    (freq, real, imag)
}