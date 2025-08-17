use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct WildPitchRng {
    rng: StdRng,
}

impl WildPitchRng {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_entropy(),
        }
    }

    pub fn with_seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T>,
    {
        self.rng.gen_range(range)
    }

    pub fn gen<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen()
    }

    pub fn gen_bool(&mut self, p: f64) -> bool {
        self.rng.gen_bool(p)
    }

    pub fn choose<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let index = self.gen_range(0..items.len());
            items.get(index)
        }
    }

    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        use rand::seq::SliceRandom;
        slice.shuffle(&mut self.rng);
    }

    // Baseball-specific random utilities
    pub fn weighted_choice(&mut self, weights: &[f64]) -> usize {
        let total: f64 = weights.iter().sum();
        let mut roll = self.gen_range(0.0..total);
        
        for (i, &weight) in weights.iter().enumerate() {
            roll -= weight;
            if roll <= 0.0 {
                return i;
            }
        }
        
        weights.len() - 1 // Fallback to last index
    }

    pub fn normal_distribution(&mut self, mean: f64, std_dev: f64) -> f64 {
        // Box-Muller transform for normal distribution
        use std::f64::consts::PI;
        
        let u1: f64 = self.gen();
        let u2: f64 = self.gen();
        
        let z0 = (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos();
        mean + std_dev * z0
    }

    // Simulate dice rolls for specific baseball situations
    pub fn roll_d6(&mut self) -> u8 {
        self.gen_range(1..=6)
    }

    pub fn roll_d20(&mut self) -> u8 {
        self.gen_range(1..=20)
    }

    pub fn roll_d100(&mut self) -> u8 {
        self.gen_range(1..=100)
    }

    // Performance variation - adds realistic randomness to player performance
    pub fn performance_modifier(&mut self, base_rating: f64, variance: f64) -> f64 {
        let modifier = self.normal_distribution(0.0, variance);
        (base_rating + modifier).clamp(0.0, 1.0)
    }

    // Hot/cold streak simulation
    pub fn streak_modifier(&mut self, streak_count: i32, max_effect: f64) -> f64 {
        let effect = (streak_count.abs() as f64 / 10.0).min(max_effect);
        if streak_count > 0 {
            effect // Hot streak - positive modifier
        } else {
            -effect // Cold streak - negative modifier
        }
    }
}

impl Default for WildPitchRng {
    fn default() -> Self {
        Self::new()
    }
}

// Seeded random state for reproducible simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeededRandom {
    seed: u64,
    counter: u64,
}

impl SeededRandom {
    pub fn new(seed: u64) -> Self {
        Self { seed, counter: 0 }
    }

    pub fn next_rng(&mut self) -> WildPitchRng {
        let combined_seed = self.seed.wrapping_add(self.counter);
        self.counter = self.counter.wrapping_add(1);
        WildPitchRng::with_seed(combined_seed)
    }

    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }
}

// Probability utilities for baseball simulation
pub struct BaseballProbabilities;

impl BaseballProbabilities {
    // MLB average probabilities (these would be configurable in full game)
    pub const WALK_RATE: f64 = 0.085;
    pub const STRIKEOUT_RATE: f64 = 0.225;
    pub const HOME_RUN_RATE: f64 = 0.035;
    pub const BATTING_AVERAGE: f64 = 0.255;
    pub const ON_BASE_PERCENTAGE: f64 = 0.325;
    pub const SLUGGING_PERCENTAGE: f64 = 0.430;
    
    pub fn adjust_for_count(base_probability: f64, balls: u8, strikes: u8) -> f64 {
        let count_modifier = match (balls, strikes) {
            (3, 0) | (3, 1) => 1.8,  // Pitcher's counts favor hitter
            (2, 0) | (2, 1) => 1.4,
            (1, 0) => 1.2,
            (0, 2) | (1, 2) => 0.6,  // Hitter's counts favor pitcher
            (0, 1) => 0.8,
            _ => 1.0,
        };
        
        (base_probability * count_modifier).clamp(0.0, 1.0)
    }

    pub fn situational_modifier(runners_on: u8, outs: u8) -> f64 {
        // Clutch situations typically favor pitchers slightly
        match (runners_on, outs) {
            (0, _) => 1.0,           // No pressure
            (_, 2) => 0.9,           // Two outs - pressure on hitter
            (runners, 0) if runners >= 2 => 0.95, // Multiple runners, no outs
            (runners, 1) if runners >= 2 => 0.92, // Multiple runners, one out
            _ => 0.98,               // Slight pressure with runners on
        }
    }
}