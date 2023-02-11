use crate::util::SplitLine;
use std::mem::take;

/// Created from an iterator of SplitLines, group the Splitlines and format the output accordingly
pub struct Grouper<S: Iterator<Item = SplitLine>> {
    source: S,
    previous_key: Option<String>,
    /// If a new key is identified, that is sent out the iterator and the current value is stored here
    value_buffer: Option<String>,
}

impl<S: Iterator<Item = SplitLine>> Iterator for Grouper<S> {
    type Item = OutLineType;
    fn next(&mut self) -> Option<Self::Item> {
        if self.value_buffer.is_some() {
            return take(&mut self.value_buffer).map(OutLineType::Value);
        }

        let split_line = self.source.next()?;
        if let Some(previous_key) = &mut self.previous_key {
            if previous_key == &split_line.key {
                Some(OutLineType::Value(split_line.value))
            } else {
                self.value_buffer = Some(split_line.value);
                *previous_key = split_line.key.clone();
                Some(OutLineType::Key(split_line.key))
            }
        } else {
            self.previous_key = Some(split_line.key.clone());
            self.value_buffer = Some(split_line.value);
            Some(OutLineType::Key(split_line.key))
        }
    }
}

impl<S: Iterator<Item = SplitLine>> From<S> for Grouper<S> {
    fn from(source: S) -> Self {
        Self {
            source,
            previous_key: None,
            value_buffer: None,
        }
    }
}

/// Enum to represent the types of lines the grouper can output
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum OutLineType {
    Key(String),
    Value(String),
}

use std::fmt;
impl fmt::Display for OutLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Key(k) => write!(f, "> {}", k),
            Self::Value(v) => write!(f, "* {}", v),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_grouper() {
        let lines: Vec<SplitLine> = vec![
            ("key 1", "13").into(),
            ("key 1", "14").into(),
            ("key 1", "15").into(),
            ("key 2", "15").into(),
            ("key 2", "16").into(),
            ("key 3", "14").into(),
            ("key 1", "13").into(),
            ("key 1", "13").into(),
        ];

        let mut grouper = Grouper::from(lines.into_iter());
        assert_eq!(grouper.next(), Some(OutLineType::Key("key 1".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("13".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("14".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("15".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Key("key 2".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("15".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("16".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Key("key 3".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("14".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Key("key 1".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("13".to_owned())));
        assert_eq!(grouper.next(), Some(OutLineType::Value("13".to_owned())));
        assert_eq!(grouper.next(), None);
    }

    #[test]
    fn formatting_out_line_type() {
        assert_eq!(
            format!("{}", OutLineType::Key("A Key".to_owned())),
            "> A Key".to_owned()
        );

        assert_eq!(
            format!("{}", OutLineType::Value("A Value".to_owned())),
            "* A Value".to_owned()
        );
    }
}
