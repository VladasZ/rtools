mod managed;

#[cfg(test)]
mod test {

    use rtools::times;

    #[test]
    fn times() {
        let ten = &mut 0;
        let twenty = &mut 0;

        for _ in 0..1000 {
            times!(10, || {
                *ten += 1;
            });

            times!(20, || {
                *twenty += 1;
            });
        }

        assert_eq!(*ten, 10);
        assert_eq!(*twenty, 20);
    }
}
