
use pest;

#[derive(Clone, Debug)]
pub struct Span<'p> {
    pub input: &'p str,
    pub start: usize,
    pub end: usize,
}

impl<'p> Span<'p> {
    pub fn as_str(&self) -> &'p str {
        &self.input[self.start..self.end]
    }

    pub fn line_no(&self) -> usize {
        let mut curr = 1;
        let mut iter = self.input[..=self.start].chars();
        while let Some(c) = iter.next_back() {
            if c == '\n' {
                curr += 1;
            }
        }

        curr
    }

    pub fn from_line_begin(&self) -> Span<'p> {
        let mut curr = self.start;
        let mut iter = self.input[..=self.start].chars();
        while let Some(c) = iter.next_back() {
            if c == '\n' {
                break;
            }
            curr -= 1;
        }
        
        Span {
            input: self.input,
            start: curr + 1,
            end: self.end,
        }
    }
    
    // Assuming we're the span of an error message,
    // convert an index into an index for annotations
    pub fn distance_from_start(&self, pos: usize) -> usize {
        assert!(pos >= self.start);
        let mut curr = 0usize;
        let mut indx = self.start;
        for c in self.input[self.start..].chars() {
            if indx == pos {
                return curr;
            }
            if c == '\n' {
                curr += 2;
            } else {
                curr += 1;
            }
            indx += 1;
        }
        unreachable!()
    }
}

pub fn new_span<'p>(pspan: pest::Span<'p>, source: &'p str) -> Span<'p> {
    Span {
        input: source,
        start: pspan.start(),
        end: pspan.end(),
    }
}
