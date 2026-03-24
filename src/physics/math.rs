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
