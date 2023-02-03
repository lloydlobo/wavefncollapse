use clap::Parser;
use rand::prelude::*;

pub type Result<T> = anyhow::Result<T>;
fn main() -> Result<()> {
    pretty_env_logger::init();
    log::info!("hello");
    run()?;
    Ok(())
}

#[derive(Parser, Debug)]
struct Opts {
    /// Dataset state array of probabilities.
    /// `-s 0.3 -s 0.5 -s 0.9`
    #[arg(short, long)]
    state: Option<Vec<f64>>,
}

/// (state, random_probability, index): ([0.51, 0.87, 0.21], 0.56, 1)
fn run() -> Result<()> {
    let opts = Opts::parse();

    let state = match opts.state {
        Some(val) => val,
        None => {
            let mut rng = thread_rng();
            (0..3).map(|_| rng.gen::<f64>()).collect()
        }
    };
    let (index, rnd_prob) = wave::collapse_wave_fn(state.clone())?;
    println!(
        "(state, random_probability, index): {:.2?}",
        (&state, &rnd_prob.parse::<f64>()?, &index)
    );

    Ok(())
}

mod wave {
    use crate::Result;
    use rand::prelude::*;

    /// # Usage
    ///
    /// $ cargo run -- -s 0.3 -s 0.5 -s 0.9
    /// (state, probability): ([0.3, 0.5, 0.9], 1)
    pub fn collapse_wave_fn(state: Vec<f64>) -> Result<(usize, String)> {
        let mut rng = thread_rng();
        let mut sum = 0f64;

        let rnd_num: f64 = rng.gen();
        for (i, probability) in state.iter().enumerate() {
            sum += *probability;
            if rnd_num < sum {
                return Ok((i, rnd_num.to_string()));
            }
        }

        Ok((state.len() - 1, rnd_num.to_string()))
    }
}
