use rand::{
    Rng, SeedableRng,
    distr::uniform::{SampleRange, SampleUniform},
};
use std::sync::Mutex;

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[derive(bevy::prelude::Resource)]
pub struct RandomNumberGenerator {
    rng: Mutex<RngCore>,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: Mutex::new(RngCore::from_os_rng()),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: Mutex::new(RngCore::seed_from_u64(seed)),
        }
    }

    pub fn next<T>(&self) -> T
    where
        rand::distr::StandardUniform: rand::prelude::Distribution<T>,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.random()
    }

    pub fn range<T>(&self, range: impl SampleRange<T>) -> T
    where
        T: SampleUniform + PartialOrd,
    {
        let mut lock = self.rng.lock().unwrap();
        lock.random_range(range)
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RandomPlugin;

impl bevy::prelude::Plugin for RandomPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(RandomNumberGenerator::new());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(1..10);
            assert!(n >= 1);
            assert!(n < 10);
        }
    }

    #[test]
    fn test_inclusive_range_bounds() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(1..=9);
            assert!(n >= 1);
            assert!(n < 10);
        }
    }

    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.range(-5000.0f32..5000.0f32);
            assert!(n.is_finite());
            assert!(n > -5000.0);
            assert!(n < 5000.0);
        }
    }

    #[test]
    fn test_reproducibility() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );
        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.range(u32::MIN..u32::MAX),
                rng.1.range(u32::MIN..u32::MAX),
            );
        });
    }
}
