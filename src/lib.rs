use rand::prelude::*;
use rand_pcg::Pcg64;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Config {
    pub num_rates: usize,
    pub img_width: u32,
    pub img_height: u32,
}

impl Config {

    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() == 1 {
            return Ok(Config { 
                num_rates: 10_000,
                img_width: 1800,
                img_height: 1200 
            });
        }
        if args.len() < 3 {
            return Err("Missing line arguments");
        } else if args.len() >= 5 {
            return Err("Too many line arguments");
        }
        
        let num_rates = args[1].parse().unwrap();
        let img_width = args[2].parse().unwrap();
        let img_height = args[3].parse().unwrap();

        return Ok(Config { 
            num_rates,
            img_width,
            img_height 
        });
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LogMapCoordinates {
    pub x_values: Vec<f32>,
    pub y_values: Vec<f32>,
}

pub fn log_mapper(x: f32, lambda: f32) -> f32 {
    return lambda * x * (1.0 - x);
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
        if let Some(_) = results.iter().find(|v| (*v - pop).abs() < POP_DIFF) {
            break;
        } else {
            results.push(pop);
        }
    }
    
    return results;
}

pub fn calculate_mapping(min_rate: f32, max_rate: f32, num_rates: usize) -> LogMapCoordinates {
    let mut rng = Pcg64::seed_from_u64(42);

    let mut points = LogMapCoordinates{
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