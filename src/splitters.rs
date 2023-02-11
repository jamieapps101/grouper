use crate::util::SplitLine;

//////////////////////////
// White space splitter //
//////////////////////////

/// Given some iterator of lines, split on whitespace to key,value pairs in a SplitLine struct
pub struct WhiteSpaceSplitter<S: Iterator<Item = String>> {
    source: S,
}

impl<S: Iterator<Item = String>> Iterator for WhiteSpaceSplitter<S> {
    type Item = SplitLine;
    fn next(&mut self) -> Option<Self::Item> {
        let line = self.source.next()?;
        if let Some((k, v)) = line.split_once([' ']) {
            Some(SplitLine {
                key: k.to_owned(),
                value: v.to_owned(),
            })
        } else {
            Some(SplitLine {
                key: line.to_owned(),
                value: "".to_owned(),
            })
        }
    }
}

impl<S: Iterator<Item = String>> From<S> for WhiteSpaceSplitter<S> {
    fn from(source: S) -> Self {
        Self { source }
    }
}

#[cfg(test)]
#[test]
fn white_space_splitter_test() {
    let lines = vec![
        "a value".to_owned(),
        "a value 2".to_owned(),
        "a value 3 ".to_owned(),
        "b value".to_owned(),
        "b".to_owned(),
        "c value 0".to_owned(),
        "".to_owned(),
        "c value 1".to_owned(),
    ];

    let mut splitter = WhiteSpaceSplitter::from(lines.into_iter());

    assert_eq!(splitter.next(), Some(("a", "value").into()));
    assert_eq!(splitter.next(), Some(("a", "value 2").into()));
    assert_eq!(splitter.next(), Some(("a", "value 3 ").into()));
    assert_eq!(splitter.next(), Some(("b", "value").into()));
    assert_eq!(splitter.next(), Some(("b", "").into()));
    assert_eq!(splitter.next(), Some(("c", "value 0").into()));
    assert_eq!(splitter.next(), Some(("", "").into()));
    assert_eq!(splitter.next(), Some(("c", "value 1").into()));
    assert_eq!(splitter.next(), None);
}

/////////////////////
// Quoted splitter //
/////////////////////

/// Given some iterator of lines, split on whitespace to key,value pairs in a SplitLine struct
#[allow(dead_code)]
pub struct QuoteSplitter<S: Iterator<Item = String>> {
    source: S,
}

impl<S: Iterator<Item = String>> Iterator for QuoteSplitter<S> {
    type Item = SplitLine;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<S: Iterator<Item = String>> From<S> for QuoteSplitter<S> {
    fn from(source: S) -> Self {
        Self { source }
    }
}

#[cfg(test)]
#[test]
#[ignore]
fn quote_splitter_test() {
    let lines = vec![
        "\"a\" value".to_owned(),
        "\"a\" value 2".to_owned(),
        "\"a\"value 3".to_owned(),
        "\"b\" value".to_owned(),
        "\"b\"".to_owned(),
        "\"c\" value 0".to_owned(),
        "".to_owned(),
        "\"c\" value 1".to_owned(),
        "c value 1".to_owned(),
    ];

    let mut splitter = QuoteSplitter::from(lines.into_iter());

    assert_eq!(splitter.next(), Some(("a", " value").into()));
    assert_eq!(splitter.next(), Some(("a", " value 2").into()));
    assert_eq!(splitter.next(), Some(("a", "value 3 ").into()));
    assert_eq!(splitter.next(), Some(("b", " value").into()));
    assert_eq!(splitter.next(), Some(("b", "").into()));
    assert_eq!(splitter.next(), Some(("c", " value 0").into()));
    assert_eq!(splitter.next(), Some(("", "").into()));
    assert_eq!(splitter.next(), Some(("c", " value 1").into()));
    assert_eq!(splitter.next(), Some(("c value 1", "").into()));
    assert_eq!(splitter.next(), None);
}
