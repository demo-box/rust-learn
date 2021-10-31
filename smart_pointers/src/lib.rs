#[cfg(test)]
mod tests {

    #[test]
    fn one() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y); // 用*解引用
    }
}
