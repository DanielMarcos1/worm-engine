use libm;

pub fn sqrt(x: f32) -> f32 {
    libm::sqrtf(x)
}

pub fn acos(x: f32) -> f32 {
    libm::acosf(x)
}

pub fn cos(x: f32) -> f32 {
    libm::cosf(x)
}

pub fn sin(x: f32) -> f32 {
    libm::sinf(x)
}
