use rand::prelude::*;
use rand_pcg::Pcg64;
use serde::{Serialize, Deserialize};
use gnuplot::{Figure, Color, PointSymbol};


#[derive(Debug)]
pub struct Config {
    pub num_rates: usize,
    pub img_width: u32,
    pub img_height: u32,
}

impl Config {

    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() == 1 {
            return Ok(Config { 
                num_rates: 10_000,
                img_width: 1800,
                img_height: 1200 
            });
        }
        if args.len() >= 5 {
            return Err("Too many line arguments");
        }

        args.next();

        let num_rates = match args.next() {
            Some(n) => n.parse().unwrap(),
            None => return Err("Missing img_width"),
        };

        let img_width = match args.next() {
            Some(n) => n.parse().unwrap(),
            None => return Err("Missing img_width"),
        };

        let img_height = match args.next() {
            Some(n) => n.parse().unwrap(),
            None => return Err("Missing img_width"),
        };

        Ok(Config { 
            num_rates,
            img_width,
            img_height,
        })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GraphCoordinates {
    pub x_values: Vec<f32>,
    pub y_values: Vec<f32>,
}

pub fn log_mapper(x: f32, lambda: f32) -> f32 {
    lambda * x * (1.0 - x)
}

pub fn calculate_end_points(initial: f32, lambda: f32) -> Vec<f32> {
    const ITERATIONS: u32 = 10_000;
    const POP_DIFF: f32 = 0.000_001;

    let mut pop = initial;

    for _ in 1..ITERATIONS {
        pop = log_mapper(pop, lambda);
    }

    let mut results: Vec<f32> = vec![pop];

    for _ in 1..ITERATIONS {
        pop = log_mapper(pop, lambda);
        // println!("rate {} generation {}", lambda, n);
        // base
        // if let Some(_) = results.iter().find(|v| (*v - pop).abs() < POP_DIFF) {
        // better
        // if let Some(_) = results.iter().find(|v| (*v - pop).abs() < POP_DIFF) {
        // best
        if results.iter().any(|v| (*v - pop).abs() < POP_DIFF) {
            break;
        } else {
            results.push(pop);
        }
    }
    
    results
}

pub fn calculate_mapping(min_rate: f32, max_rate: f32, num_rates: usize) -> GraphCoordinates {
    let mut rng = Pcg64::seed_from_u64(42);

    let mut points = GraphCoordinates{
        x_values: vec![],
        y_values: vec![],
    };

    let increment = (max_rate - min_rate) / (num_rates as f32);

    /* Learn to implement iterators. 
     *
     * This would be better/more idiomatic if using iterators
     * but I still don't know how to properly use them
     * 
     * get confused when trying to get a variable length vector
     * for each of the rates used in the calculation
     * a rate could have from 1 y values to an a priori undefined
     * number of y values, since the nature of the function is chaotic
     * 
     * how to implement this?
     */
    
    for i in 0..num_rates {
        let rate = increment.mul_add(i as f32, min_rate);
        let pop: f32 = rng.gen();
        // println!("rate number {} with rate {}", i, rate);
        let mut res = calculate_end_points(pop, rate);

        points.x_values.append(&mut vec![rate; res.len()]);
        points.y_values.append(&mut res);
    }

    points

}

pub fn one_rate_evolution(initial: f32, lambda: f32) -> GraphCoordinates {
    const ITERATIONS: usize = 500;
    
    let mut points = GraphCoordinates {
        x_values: vec![0.; ITERATIONS + 1],
        y_values: vec![initial; ITERATIONS + 1],
    };

    let mut pop = initial;

    // code using iterators
    for i in 1..=ITERATIONS {
        pop = log_mapper(pop, lambda);

        points.x_values[i] = i as f32;
        points.y_values[i] = pop;

    };

    points
}

pub fn output_graph(
    points: &GraphCoordinates, title: &str, graph_type: &str,
    outpath: &str, img_width: u32, img_height: u32
) {
    let mut fig = Figure::new();
    fig.set_title(title);
    if graph_type == "lines" {
        fig.axes2d().lines(
            &points.x_values,
            &points.y_values,
            &[Color("black")]
        );
    } else if graph_type == "points" {
        fig.axes2d().points(
            &points.x_values,
            &points.y_values,
            &[PointSymbol('.'), Color("black")]
        );
    }
    fig.save_to_png(outpath, img_width, img_height).unwrap();
    fig.close();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn logistic_map_works() {
        let x: f32 = 0.4;
        let l: f32 = 2.8;
        let result = log_mapper(x, l);
        assert_eq!(result, l * x * (1.0 - x));
    }
}