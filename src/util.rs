/// A representation of a split line
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct SplitLine {
    pub key: String,
    pub value: String,
}

#[cfg(test)]
impl From<(&str, &str)> for SplitLine {
    fn from((s1, s2): (&str, &str)) -> SplitLine {
        SplitLine {
            key: s1.to_owned(),
            value: s2.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_split_line_from() {
        assert_eq!(
            SplitLine {
                key: "a key".to_owned(),
                value: "a value".to_owned()
            },
            ("a key", "a value").into()
        )
    }
}
