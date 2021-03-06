#![feature(test)]
extern crate rs_poker;
extern crate test;
extern crate rand;

use rs_poker::holdem::StartingHand;


#[bench]
fn all_starting(b: &mut test::Bencher) {
    b.iter(StartingHand::all)
}

#[bench]
fn iter_everything(b: &mut test::Bencher) {
    b.iter(|| -> usize {
               StartingHand::all()
                   .iter()
                   .map(|sh| -> usize { sh.possible_hands().len() })
                   .sum()
           })
}
