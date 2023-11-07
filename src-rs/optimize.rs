mod export;

use indicatif::ProgressBar;
use rayon::prelude::*;
use solar_oven::*;
use std::sync::{Arc, Mutex};

fn main() {
    let all = Arc::new(Mutex::new(Vec::new()));

    let variants = variants();

    let variant_count = variants.len();
    let pb = ProgressBar::new(variant_count as u64);

    variants
        .into_par_iter()
        .for_each_with(all.clone(), |all, (oven, init)| {
            let (best_design, score) = oven.best_design(init);

            all.lock().unwrap().push((best_design, score));

            pb.inc(1);
        });

    pb.finish();

    let mut all = all.lock().unwrap();
    all.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!("\nBest 3 designs of {variant_count}:\n");
    for (design, score) in all.iter().take(3) {
        println!("score: {score}");
        println!(
            "cost based performance index: {}",
            (design.predicted_tio() - AMBIENT) / design.total_cost()
        );
        println!("{}", design);
    }
}
