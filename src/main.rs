use std::fs::File;
use std::path::Path;
use std::io::{BufReader, BufRead};

use plotters::prelude::*;

fn main() {
    let (freq, real, imag) = read_file("./dat/20201202-KS-KS326-PEIS-10mV-1MHz-100mHz-20ptsprdec-charged_symmetric_inert_ref_LMO-VSP_C03.mpt");
    plot(freq, real, imag);
}

fn read_file(filename: &str) -> (Vec<f64>, Vec<f64>, Vec<f64>) {

    let f = File::open(filename); //Could have added an .unwrap() to this, but handling it manually with match gives more learning!
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Cannot open file: {:?}", error),
    };
    let f1 = File::open(filename); //Could have added an .unwrap() to this, but handling it manually with match gives more learning!
    let f1 = match f1 {
        Ok(file) => file,
        Err(error) => panic!("Cannot open file: {:?}", error),
    };
    let data = BufReader::new(f);
    let data2 = BufReader::new(f1);


    //Initialize vectors
    let mut freq: Vec<f64> = vec![];
    let mut real: Vec<f64> = vec![];
    let mut imag: Vec<f64> = vec![];

    let mut n_head = 1u8;

    // First iterate in order to find number of header elements to skip
    for line  in data.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => String::new(),
        };

        if line.contains("Nb header lines :") {
            let n_headerlines: Vec<&str> = line.split(":").collect();
            let n_headerlines = n_headerlines.last().unwrap();
            n_head = n_headerlines.trim().parse::<u8>().unwrap();
            break;
        }
    }
    //println!("# Headerlines: {}", &n_head);

    // Iterate over lines in order to add to vectors
    for line in data2.lines().skip(n_head as usize) {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("Can't read line in file: {:?}", error);
                String::new()
            }
        };
        //println!("{:?}", &line);
        let linevec: Vec<&str> = line.split("\t").collect();

        let cur_freq = linevec[0].trim().replace(",", ".").parse::<f64>().unwrap();
        freq.push(cur_freq);

        let cur_real = linevec[1].trim().replace(",", ".").parse::<f64>().unwrap();
        freq.push(cur_real);

        let cur_imag = linevec[2].trim().replace(",", ".").parse::<f64>().unwrap();
        freq.push(cur_imag);
    }
    //println!("{:?}", freq);

    (freq, imag, real)
    
    

}

fn plot(freq: Vec<f64>, real: Vec<f64>, imag: Vec<f64>) {
    println!("plotfunc");
    let drawing_area = BitMapBackend::new("images/2.0.png", (1024, 768))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.draw_series(
        LineSeries::new((1..100).map(|x| (x, 100-x)), &BLACK),
    ).unwrap();
}