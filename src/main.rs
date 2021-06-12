use std::fs::File;
use std::env;
use std::process;

fn main() {
    let min_rate = 0.0;
    let max_rate = 4.0;

    let config = logistic_map::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error encountered: {}", err);
        process::exit(2);
    });

    // calcula el mapping completo, encontrando los puntos estables para todos los R en el itnervalo
    let points_to_plot = logistic_map::calculate_mapping(
        min_rate,
        max_rate,
        config.num_rates
    );

    let file = File::create("./logistic_map_values.json").unwrap();
    serde_json::to_writer(&file, &points_to_plot)
        .expect("Error when serializing.");

    let title_1 = format!("Logistic Map Bifurcation Plot with {} rates", config.num_rates);
    logistic_map::output_graph(
        &points_to_plot, &title_1, "points",
        "./logistic_map_bifurcation.png", config.img_width, config.img_height
    );

    let ejercicio_1_a_1 = logistic_map::one_rate_evolution(0.2, 2.);
    let title = "Evolucion con poblacion inicial 0.2 y tasa 2.0";
    logistic_map::output_graph(
        &ejercicio_1_a_1, &title, "lines",
        "./evolucion_pob-0.2_rate-2.png", config.img_width, config.img_height
    );

    let ejercicio_1_a_2 = logistic_map::one_rate_evolution(0.9, 2.);
    let title = "Evolucion con poblacion inicial 0.9 y tasa 2.0";
    logistic_map::output_graph(
        &ejercicio_1_a_2, &title, "lines",
        "./evolucion_pob-0.9_rate-2.png", config.img_width, config.img_height
    );

    let ejercicio_1_b_1 = logistic_map::one_rate_evolution(0.2, 3.1);
    let title = "Evolucion con poblacion inicial 0.2 y tasa 3.1";
    logistic_map::output_graph(
        &ejercicio_1_b_1, &title, "lines",
        "./evolucion_pob-0.2_rate-3.1.png", config.img_width, config.img_height
    );

    let ejercicio_1_d_2 = logistic_map::one_rate_evolution(0.2, 4.);
    let title = "Evolucion con poblacion inicial 0.2 y tasa 3.1";
    logistic_map::output_graph(
        &ejercicio_1_d_2, &title, "lines",
        "./evolucion_pob-0.2_rate-4.png", config.img_width, config.img_height
    );

    let ejercicio_1_d_2 = logistic_map::one_rate_evolution(0.200000001, 4.);
    let title = "Evolucion con poblacion inicial 0.2 y tasa 3.1";
    logistic_map::output_graph(
        &ejercicio_1_d_2, &title, "lines",
        "./evolucion_pob-0.200000001_rate-4.png", config.img_width, config.img_height
    );

    let ejercicio_1_d_2 = logistic_map::one_rate_evolution(0.9, 4.);
    let title = "Evolucion con poblacion inicial 0.2 y tasa 3.1";
    logistic_map::output_graph(
        &ejercicio_1_d_2, &title, "lines",
        "./evolucion_pob-0.9_rate-4.png", config.img_width, config.img_height
    );
}
