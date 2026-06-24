use libm;

pub trait DeterministicMath {
    fn d_sqrt(self) -> Self;
    fn d_acos(self) -> Self;
    fn d_abs(self) -> Self;
    fn d_min(self, other: Self) -> Self;
    fn d_max(self, other: Self) -> Self;
}

impl DeterministicMath for f32 {
    #[inline(always)]
    fn d_sqrt(self) -> Self {
        libm::sqrtf(self)
    }

    #[inline(always)]
    fn d_acos(self) -> Self {
        libm::acosf(self)
    }

    #[inline(always)]
    fn d_abs(self) -> Self {
        libm::fabsf(self)
    }

    #[inline(always)]
    fn d_min(self, other: Self) -> Self {
        libm::fminf(self, other)
    }

    #[inline(always)]
    fn d_max(self, other: Self) -> Self {
        libm::fmaxf(self, other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_sqrt() {
        let val = 2.0_f32;
        let result = val.d_sqrt();
        let expected = 1.4142135;
        assert!((result - expected).d_abs() < 1e-6);
    }

    #[test]
    fn test_deterministic_acos() {
        let val = 0.5_f32;
        let result = val.d_acos();
        let expected = 1.0471976;
        assert!((result - expected).d_abs() < 1e-6);
    }

    #[test]
    fn test_deterministic_abs() {
        let val = -3.14_f32;
        assert_eq!(val.d_abs(), 3.14);
    }

    #[test]
    fn test_deterministic_min() {
        assert_eq!(1.0_f32.d_min(2.0), 1.0);
        assert_eq!(2.0_f32.d_min(1.0), 1.0);
    }

    #[test]
    fn test_deterministic_max() {
        assert_eq!(1.0_f32.d_max(2.0), 2.0);
        assert_eq!(2.0_f32.d_max(1.0), 2.0);
    }
}
