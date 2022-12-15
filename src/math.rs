use std::fmt::Debug;

#[const_trait]
pub trait IntoF32: Copy + Sized + Sync + Send + Debug + 'static {
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

pub const fn mm_to_inch<T: ~const IntoF32>(mm: T) -> f32 {
    mm.into_f32() * 0.0393701
}
