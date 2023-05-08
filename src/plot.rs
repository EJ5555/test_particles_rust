extern crate bincode;

use std::fs::File;
use std::path::Path;
//use std::io::{Read, Result};

use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "graphs/animation.gif";

pub fn load_data<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<[f64; 3]>> {//{
    let mut result: Vec< [f64; 3] > = Vec::new();

    if let Ok(file) = File::open(path) {
        let mut reader = csv::Reader::from_reader(file);
        for record in reader.deserialize() {  //  deserialize(){
            if let Ok(element) = record {
            result.push(element);
            println!("{:?}", element); // of type [f64; 3]
            } else {
                continue;
            }
        }
        
    }
    Ok(result)
}


pub fn create_plot<P: AsRef<Path>>(data_path: P) -> Result<(), Box<dyn std::error::Error>> { //

    if let Ok(data) = load_data(data_path) {

    let root = BitMapBackend::gif(OUT_FILE_NAME, (800, 800), 50)?.into_drawing_area();
    
    for (i, cur_data) in data.iter().enumerate() {
        let mut cur_data_vec: Vec<[f64; 3]> = Vec::new();
        cur_data_vec.push(*cur_data);
        
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("Title (n_iter = {})", i ),
                ("sans-serif", 50),
            )
            .build_cartesian_3d(-5.0..5.0, -5.0..5.0, -5.0..5.0)?;

        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });

        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.5))
            .max_light_lines(5)
            .draw()?;


        chart.draw_series(
            cur_data_vec.iter()
                .map(|s| Circle::new((s[0],s[1],s[2]), 3, BLUE.filled())),
        )?;


        root.present().expect("Unable to write result to file, please make sure dir exists in the current dir");
    }
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
    } else {
        println!("Couldn't load file... :(");

        Ok(())
    }
}

#[test]
fn entry_point() {
    create_plot().unwrap()
}
