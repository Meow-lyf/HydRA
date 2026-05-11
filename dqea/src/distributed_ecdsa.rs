pub fn commit(srs: &Srs, poly: &Poly) -> Result<Commitment, PcError> {
    let degree = poly.degree();
    if degree > srs.max_degree {
        return Err(PcError::DegreeTooLarge {
            degree,
            max_degree: srs.max_degree,
        });
    }

    let mut acc = G1Projective::zero();
    for (l, coeff) in poly.coeffs.iter().enumerate() {
        if coeff.is_zero() {
            continue;
        }
        let basis = srs.gamma_g_powers[l];
        acc += basis.mul_bigint(coeff.into_bigint());
    }

    Ok(Commitment(acc.into_affine()))
}
