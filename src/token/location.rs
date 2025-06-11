#[derive(Debug, PartialEq)]
pub struct Location {
    start: usize,
    end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Location {
        Location {
            start,
            end
        }
    }
}