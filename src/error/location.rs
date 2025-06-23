use std::cmp::{max, min};
use crate::ast::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
    pub fn new(start: usize, end: usize) -> Location {
        Location {
            start,
            end
        }
    }
    
    pub fn from_locations(locations: Vec<Location>) -> Location {
        locations.iter().fold(Location::new(usize::MAX, usize::MIN), |mut acc, x| {
            acc.start = min(acc.start, x.start);
            acc.end = max(acc.end, x.end);
            acc
        })
    }
}