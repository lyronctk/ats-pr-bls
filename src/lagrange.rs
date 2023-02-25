use crate::bls::{ECPoint, ECScalar};
use curv::BigInt;

pub fn lagrange_coeff_f0<T: ECPoint>(points: &Vec<(usize, T)>, j: usize) -> T::Scalar {
    let fe2_xj: T::Scalar = ECScalar::from(&BigInt::from(points[j].0 as u32));
    let mut prod: T::Scalar = ECScalar::from(&BigInt::from(1));
    for (i, _) in points.iter().enumerate() {
        if i == j {
            continue;
        }
        let fe2_xi: T::Scalar = ECScalar::from(&BigInt::from(points[i].0 as u32));
        let diff: T::Scalar = fe2_xi.sub(&fe2_xj.get_element());
        prod = prod * fe2_xi * diff.invert();
    }
    prod
}

pub fn lagrange_interpolate_f0<T: ECPoint + Copy>(points: &Vec<(usize, T)>) -> T {
    let mut summation: T = T::generator().sub_point(&T::generator().get_element());
    for (j, p) in points.iter().enumerate() {
        let lambda: T::Scalar = lagrange_coeff_f0(points, j);
        summation = summation + p.1 * lambda;
    }
    summation
}
