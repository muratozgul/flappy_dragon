use rand::{distr::uniform::{SampleUniform, SampleRange}, Rng, SeedableRng};

#[cfg(all(not(feature = "pcg"), not(feature = "xorshift")))]
type RngCore = rand::prelude::StdRng;

#[cfg(feature = "pcg")]
type RngCore = rand_pcg::Pcg64Mcg;

#[cfg(feature = "xorshift")]
type RngCore = rand_xorshift::XorShiftRng;

#[derive(bevy::prelude::Resource)]
pub struct RandomNumberGenerator {
    rng: RngCore,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: RngCore::from_os_rng(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: RngCore::seed_from_u64(seed),
        }
    }

    pub fn range<T>(&mut self, range: impl SampleRange<T>) -> T
    where T: SampleUniform + PartialOrd
    {
        self.rng.random_range(range)
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