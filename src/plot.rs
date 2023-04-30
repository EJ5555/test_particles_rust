use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "graphs/animation.gif";
pub fn create_plot(data: [crate::solver::Position; 1000]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::gif(OUT_FILE_NAME, (800, 800), 100)?.into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Title (n_iter = {})", 5),
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
        data.iter()
            .map( |crate::solver::Position(x,y,z)| Circle::new((*x, *y, *z), 3, BLUE.filled())),
    )?;

    root.present().expect("Unable to write result to file, please make sure dir exists in the current dir");

    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[test]
fn entry_point() {
    create_plot().unwrap()
}
