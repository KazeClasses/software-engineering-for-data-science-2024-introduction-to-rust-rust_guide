use rand::{prelude::Distribution, rngs::StdRng, Rng, SeedableRng};
use statrs::distribution::MultivariateNormal;



pub struct State<const N_DIM: usize> {
    rng: StdRng,
    pub arr: [f64, N_DIM],
    proposal_distribution: MultivariateNormal,
}

fn log_likelihood<const N_DIM: usize>(arr: &[f64]) -> f64 {
    arr.iter().take(N_DIM).map(|&x| x.powi(2)).sum::<f64>() * 0.5
}

impl<const N_DIM: usize> State<N_DIM> {
    pub fn new(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);
        let arr = [0.0; N_DIM];
        let mean = vec![0.0; N_DIM]; 
        let cov = vec![1.0; N_DIM]; 
        let proposal_distribution = MultivariateNormal::new(mean, cov).unwrap();

        Self {
            rng,
            arr,
            proposal_distribution,
        }
    }

    pub fn take_step(&mut self) {
    // Fill this
    }
}
