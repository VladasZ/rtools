pub trait IntoF32: Copy + Sized {
    fn into_f32(self) -> f32;
}

impl IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

pub fn clamped(value: impl IntoF32) -> f32 {
    clamped_by(0, 1, value)
}

pub fn clamped_by(low: impl IntoF32, high: impl IntoF32, value: impl IntoF32) -> f32 {
    let value = value.into_f32();
    let low = low.into_f32();
    let hight = high.into_f32();
    if value < low {
        low
    } else if value > hight {
        hight
    } else {
        value
    }
}

pub fn mm_to_inch(mm: impl IntoF32) -> f32 {
    mm.into_f32() * 0.0393701
}
