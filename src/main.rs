use logistic_map;
use serde_json;
use std::fs::File;

const NUM_RATES: usize = 10_000;

fn main() {
    let min_rate = 0.0;
    let max_rate = 4.0;

    let points_to_plot = logistic_map::calculate_mapping(
        min_rate,
        max_rate,
        NUM_RATES
    );

    let file = File::create("./logistic_map_values.json").unwrap();
    serde_json::to_writer(&file, &points_to_plot)
        .expect("Error when serializing.");


/*     let mut fig = Figure::new();
    fig.set_title("Logistic Mapping");
    fig.axes2d().points(
        &points_to_plot.x_values,
        &points_to_plot.y_values,
        &[PointSymbol('.')]
    );
    fig.save_to_png("./logistic_map_bifurcation.png", 30000, 15000).unwrap();
    fig.close(); */
}
