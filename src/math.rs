pub trait IntoF32: Copy + Sized + Sync + Send + 'static {
    fn into_f32(self) -> f32;
}

impl const IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for usize {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for f32 {
    fn into_f32(self) -> f32 {
        self
    }
}

impl const IntoF32 for f64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

pub const fn clamped<T: ~const IntoF32>(value: T) -> f32 {
    clamped_by(0, 1, value)
}

pub const fn clamped_by<Low: ~const IntoF32, High: ~const IntoF32, Value: ~const IntoF32>(
    low: Low,
    high: High,
    value: Value,
) -> f32 {
    let value = value.into_f32();
    let low = low.into_f32();
    let high = high.into_f32();
    if value < low {
        low
    } else if value > high {
        high
    } else {
        value
    }
}

pub const fn mm_to_inch<T: ~const IntoF32>(mm: T) -> f32 {
    mm.into_f32() * 0.0393701
}
