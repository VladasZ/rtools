use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Unwrap<T> {
    value: Option<T>,
}

impl<T> Unwrap<T> {
    pub const fn const_default() -> Self {
        Self { value: None }
    }

    pub fn new(val: T) -> Self {
        Self { value: Some(val) }
    }

    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    pub fn set(&mut self, val: T) -> Option<T> {
        let result = self.take();
        self.value = Some(val);
        result
    }

    pub fn take(&mut self) -> Option<T> {
        self.value.take()
    }
}

impl<T> Deref for Unwrap<T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self.value.as_ref() {
            Some(rf) => rf,
            None => {
                panic!("Invalid Unwrap<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T> DerefMut for Unwrap<T> {
    fn deref_mut(&mut self) -> &mut T {
        match self.value.as_mut() {
            Some(rf) => rf,
            None => {
                panic!("Invalid Unwrap<{}>", std::any::type_name::<T>());
            }
        }
    }
}

impl<T> From<T> for Unwrap<T> {
    fn from(val: T) -> Self {
        Self { value: val.into() }
    }
}

impl<T> Default for Unwrap<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

#[cfg(test)]
mod test {
    use crate::Unwrap;

    #[test]
    fn unwrap() {
        let val = Unwrap::from(5);
        assert_eq!(5, *val);
        dbg!(val);
        let no = Unwrap::<u32>::default();
        assert!(no.is_none());
    }
}
