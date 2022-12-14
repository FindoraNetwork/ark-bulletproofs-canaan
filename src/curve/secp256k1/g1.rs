use crate::curve::secp256k1::{Fq, Fr};
use ark_ec::{
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
    ModelParameters, SWModelParameters,
};
use ark_ff::field_new;
use ark_std::Zero;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

pub type G1Affine = GroupAffine<Parameters>;
pub type G1Projective = GroupProjective<Parameters>;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 1
    const COFACTOR: &'static [u64] = &[0x1];

    /// COFACTOR_INV = COFACTOR^{-1} mod r = 1
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "1");
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 7
    const COEFF_B: Fq = field_new!(Fq, "7");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X = 55066263022277343669578718895168534326250603453777594175500187360389116729240
pub const G1_GENERATOR_X: Fq = field_new!(
    Fq,
    "55066263022277343669578718895168534326250603453777594175500187360389116729240"
);

/// G1_GENERATOR_Y = 32670510020758816978083085130507043184471273380659243275938904335757337482424
pub const G1_GENERATOR_Y: Fq = field_new!(
    Fq,
    "32670510020758816978083085130507043184471273380659243275938904335757337482424"
);

#[cfg(test)]
mod test {
    use crate::curve::secp256k1::g1::{G1Affine, G1Projective, Parameters};
    use ark_algebra_test_templates::{
        curves::{curve_tests, sw_tests},
        groups::group_test,
        msm::test_var_base_msm,
    };
    use ark_ec::AffineCurve;
    use ark_std::rand::Rng;

    #[test]
    fn test_g1_projective_curve() {
        curve_tests::<G1Projective>();
    }

    #[test]
    fn test_g1_projective_sw() {
        sw_tests::<Parameters>();
    }

    #[test]
    fn test_g1_affine_curve() {
        test_var_base_msm::<G1Affine>();
        ark_algebra_test_templates::msm::test_chunked_pippenger::<G1Affine>();
    }

    #[test]
    fn test_g1_projective_group() {
        let mut rng = ark_std::test_rng();
        let a: G1Projective = rng.gen();
        let b: G1Projective = rng.gen();
        group_test(a, b);
    }

    #[test]
    fn test_g1_generator() {
        let generator = G1Affine::prime_subgroup_generator();
        assert!(generator.is_on_curve());
    }
}
