use ark_poly::Polynomial;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::vec::Vec;

use crate::pcs::*;
use crate::utils::poly;
use crate::Poly;

#[derive(Clone, PartialEq, Eq, Debug, CanonicalSerialize, CanonicalDeserialize)]
pub struct WrappedPolynomial<F: PrimeField>(pub Poly<F>);

impl<F: PrimeField> WrappedPolynomial<F> {
    fn evaluate(&self, x: &F) -> F {
        self.0.evaluate(x)
    }
}

impl<F: PrimeField> Add<Self> for WrappedPolynomial<F> {
    type Output = WrappedPolynomial<F>;

    fn add(self, other: WrappedPolynomial<F>) -> Self::Output {
        WrappedPolynomial(self.0 + other.0)
    }
}

impl<F: PrimeField> Sub<Self> for WrappedPolynomial<F> {
    type Output = WrappedPolynomial<F>;

    fn sub(self, other: WrappedPolynomial<F>) -> Self::Output {
        WrappedPolynomial(self.0 - other.0)
    }
}

impl<F: PrimeField> Sum<Self> for WrappedPolynomial<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|a, b| a + b).unwrap()
    }
}

impl<F: PrimeField> Mul<F> for WrappedPolynomial<F> {
    type Output = WrappedPolynomial<F>;

    fn mul(self, by: F) -> Self {
        (&self).mul(by)
    }
}

impl<F: PrimeField> Commitment<F> for WrappedPolynomial<F> {
    fn mul(&self, by: F) -> Self {
        WrappedPolynomial(&self.0 * by)
    }

    fn combine(coeffs: &[F], commitments: &[Self]) -> Self {
        let polys = commitments
            .to_vec()
            .into_iter()
            .map(|c| c.0)
            .collect::<Vec<_>>();
        let combined = poly::sum_with_coeffs(coeffs.to_vec(), &polys);
        WrappedPolynomial(combined)
    }
}

impl CommitterKey for () {
    fn max_degree(&self) -> usize {
        usize::MAX >> 1
    }
}

impl VerifierKey for () {
    fn max_points(&self) -> usize {
        1
    }
}

impl RawVerifierKey for () {
    type VK = ();

    fn prepare(&self) -> () {
        ()
    }
}

impl PcsParams for () {
    type CK = ();
    type VK = ();
    type RVK = ();

    fn ck(&self) -> () {
        ()
    }

    fn vk(&self) -> () {
        ()
    }

    fn raw_vk(&self) -> () {
        ()
    }
}

#[derive(Clone)]
pub struct IdentityCommitment {}

impl<F: PrimeField> PCS<F> for IdentityCommitment {
    type C = WrappedPolynomial<F>;
    type Proof = ();
    type CK = ();
    type VK = ();
    type Params = ();

    fn setup<R: Rng>(_max_degree: usize, _rng: &mut R) -> Self::Params {
        ()
    }

    fn commit(_ck: &(), p: &Poly<F>) -> Result<Self::C, ()> {
        Ok(WrappedPolynomial(p.clone()))
    }

    fn open(_ck: &(), _p: &Poly<F>, _x: F) -> Result<Self::Proof, ()> {
        Ok(())
    }

    fn verify(_vk: &(), c: Self::C, x: F, z: F, _proof: Self::Proof) -> Result<(), ()> {
        (c.evaluate(&x) == z).then(|| ()).ok_or(())
    }
}
