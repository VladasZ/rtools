pub trait Apply<T> {
    fn apply(self, action: impl FnMut(&mut T));
    fn apply2<U, Second: IntoIterator<Item = U>>(
        self,
        second: Second,
        action: impl FnMut(&mut T, U),
    );
}

impl<T, I: IntoIterator<Item = T>> Apply<T> for I {
    fn apply(self, mut action: impl FnMut(&mut T)) {
        for mut item in self.into_iter() {
            action(&mut item)
        }
    }

    fn apply2<U, Second: IntoIterator<Item = U>>(
        self,
        second: Second,
        mut action: impl FnMut(&mut T, U),
    ) {
        for (mut item, second) in self.into_iter().zip(second.into_iter()) {
            action(&mut item, second);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::apply::Apply;

    #[test]
    fn apply_arr() {
        let mut ve = vec![];
        [1, 2, 3, 4, 5].apply(|a| {
            ve.push(*a);
        });
        assert_eq!(&ve, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn apply_tuple() {
        let mut num = vec![];
        let mut ch = vec![];
        [(1, '5'), (2, '4'), (3, '3'), (4, '2'), (5, '1')].apply(|(n, c)| {
            num.push(*n);
            ch.push(c.clone());
        });
        assert_eq!(&num, &[1, 2, 3, 4, 5]);
        assert_eq!(&ch, &['5', '4', '3', '2', '1']);
    }

    #[test]
    fn apply2_arr() {
        let mut num = vec![];
        let mut ch = vec![];
        [1, 2, 3, 4, 5].apply2(['5', '4', '3', '2', '1'], |n, c| {
            num.push(*n);
            ch.push(c);
        });
        assert_eq!(&num, &[1, 2, 3, 4, 5]);
        assert_eq!(&ch, &['5', '4', '3', '2', '1']);
    }
}
