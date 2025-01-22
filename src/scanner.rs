pub struct Scanner<'a> {
    start: &'a str,
    current: &'a str,
    line: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            start: source,
            current: source,
            line: 1,
        }
    }
}
