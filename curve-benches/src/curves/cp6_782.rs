use rand::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::ops::{AddAssign, MulAssign, SubAssign};

use ark_cp6_782::{
    fq::Fq, fq3::Fq3, fr::Fr, Fq6, G1Affine, G1Projective as G1, G2Affine, G2Projective as G2,
    CP6_782,
};
use ark_ec::{PairingEngine, ProjectiveCurve};
use ark_ff::{
    biginteger::{BigInteger384 as FrRepr, BigInteger832 as FqRepr},
    BigInteger, Field, PrimeField, SquareRootField, UniformRand,
};

mod g1 {
    use super::*;
    ec_bench!(G1, G1Affine);
}
mod g2 {
    use super::*;
    ec_bench!(G2, G2Affine);
}

f_bench!(1, Fq3, Fq3, fq3);
f_bench!(2, Fq6, Fq6, fq6);
f_bench!(Fq, Fq, FqRepr, FqRepr, fq);
f_bench!(Fr, Fr, FrRepr, FrRepr, fr);
pairing_bench!(CP6_782, Fq6, affine_v);
