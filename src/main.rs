use std::fs::File;
use gnuplot::{Figure, Color, PointSymbol};
use std::env;
use std::process;

fn main() {
    let min_rate = 0.0;
    let max_rate = 4.0;

    let args: Vec<String> = env::args().collect();

    let config = logistic_map::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Error encountered: {}", err);
        process::exit(2);
    });

    let points_to_plot = logistic_map::calculate_mapping(
        min_rate,
        max_rate,
        config.num_rates
    );

    let file = File::create("./logistic_map_values.json").unwrap();
    serde_json::to_writer(&file, &points_to_plot)
        .expect("Error when serializing.");

    let mut fig = Figure::new();
    let title = format!("Logistic Map Bifurcation Plot with {} rates", config.num_rates);
    fig.set_title(&title);
    fig.axes2d().points(
        &points_to_plot.x_values,
        &points_to_plot.y_values,
        &[PointSymbol('.'), Color("black")]
    );
    fig.save_to_png("./logistic_map_bifurcation.png", config.img_width, config.img_height).unwrap();
    fig.close();
}
