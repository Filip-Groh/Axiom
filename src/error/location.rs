use std::cmp::{max, min, Ordering};
use lsp_types::{
    Position as LSPPosition,
    Range as LSPRange,
};

pub trait Location {
    fn location(&self) -> Range;
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Position {
        Position {
            line,
            column,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.line, self.column) == (other.line, other.column) {
            return Some(Ordering::Equal)
        } else if self.line < other.line || (self.line == other.line && self.column < other.column) {
            return Some(Ordering::Less)
        } else if self.line > other.line || (self.line == other.line && self.column > other.column) {
            return Some(Ordering::Greater)
        }
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Range {
        Range {
            start,
            end,
        }
    }

    pub fn from_ranges(ranges: Vec<Range>) -> Range {
        ranges.iter().fold(Range::new(Position::new(usize::MAX, usize::MAX), Position::new(usize::MIN, usize::MIN)), |mut acc, x| {
            acc.start = min(acc.start, x.start.clone());
            acc.end = max(acc.end, x.end.clone());
            acc
        })
    }
}

impl Into<LSPRange> for Range {
    fn into(self) -> LSPRange {
        LSPRange {
            start: LSPPosition {
                line: self.start.line as u32,
                character: self.start.column as u32,
            },
            end: LSPPosition {
                line: self.end.line as u32,
                character: self.end.column as u32 + 1,
            }
        }
    }
}