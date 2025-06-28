#[allow(unused)]
fn intro() -> &'static str {
    //  fix me 👇
    "I'm ready to build a concurrent ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(
            intro(),
            "I'm ready to build a concurrent ticket management system!"
        );
    }
}
