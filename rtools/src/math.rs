use std::fmt::{Debug, Display};

#[const_trait]
pub trait IntoF32: Copy + Sized + Sync + Send + Debug + Display + 'static {
    fn into_f32(self) -> f32;
}

impl const IntoF32 for i32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for i64 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u32 {
    fn into_f32(self) -> f32 {
        self as f32
    }
}

impl const IntoF32 for u64 {
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

pub fn mm_to_inch<T: IntoF32>(mm: T) -> f32 {
    mm.into_f32() * 0.0393701
}

#[cfg(test)]
mod test {
    use crate::{mm_to_inch, IntoF32};

    #[test]
    fn test() {
        assert_eq!(1.968505, mm_to_inch(50));

        assert_eq!(15_f32, 15_i32.into_f32());
        assert_eq!(15_f32, 15_i64.into_f32());
        assert_eq!(15_f32, 15_u32.into_f32());
        assert_eq!(15_f32, 15_u64.into_f32());
        assert_eq!(15_f32, 15_usize.into_f32());
        assert_eq!(15_f32, 15_f64.into_f32());
        assert_eq!(15_f32, 15_f32.into_f32());
    }
}
