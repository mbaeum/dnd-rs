#![cfg_attr(feature = "unstable", feature(test))]

#[cfg(all(feature = "unstable", test))]
mod bench {
    extern crate test;
    use self::test::Bencher;

    use random_spells_cli::core::entity::dice_set::Dice;

    static BIG_BENCH_SIZE_DICE_COUNT: u64 = 1_000_000;
    static SMALL_BENCH_SIZE_DICE_COUNT: u64 = 2;

    #[bench]
    fn bench_dice_roll_big(b: &mut Bencher) {
        let dice = Dice::new(BIG_BENCH_SIZE_DICE_COUNT, 20, Some(2));
        b.iter(|| dice.roll());
    }

    #[bench]
    fn bench_dice_roll_small(b: &mut Bencher) {
        let dice = Dice::new(SMALL_BENCH_SIZE_DICE_COUNT, 20, Some(2));
        b.iter(|| dice.roll());
    }
}
