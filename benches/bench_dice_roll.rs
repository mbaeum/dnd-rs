#![feature(test)]

extern crate test;

use random_spells_cli::core::entity::dice_set::Dice;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    static BENCH_SIZE_DICE_COUNT: u64 = 1_000_000;

    #[bench]
    fn bench_dice_roll(b: &mut Bencher) {
        let dice = Dice::new(BENCH_SIZE_DICE_COUNT, 20, Some(2));
        b.iter(|| dice.roll());
    }
}
