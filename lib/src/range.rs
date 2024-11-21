#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range(usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InclusionRange {
    initial: Range,
    inclusions: Vec<Range>,
}

impl InclusionRange {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            initial: Range::new(from, to),
            inclusions: vec![Range::new(from, to)],
        }
    }

    pub fn update_more_than(&mut self, more_than: usize) {
        let last = self.inclusions.last().expect("must be a last");
        if more_than > last.1 {
            self.inclusions
                .push(Range::new(more_than + 1, self.initial.to()));
        } else {
            if let Some(r) = self.inclusions.iter_mut().find(|r| r.contains(more_than)) {
                r.update_more_than(more_than);
            }
        }
    }

    pub fn update_less_than(&mut self, less_than: usize) {
        let first = self.inclusions.first().expect("must be a first");
        if less_than < first.0 {
            self.inclusions
                .push(Range::new(self.initial.from(), less_than));
        } else {
            if let Some(r) = self.inclusions.iter_mut().find(|r| r.contains(less_than)) {
                r.update_less_than(less_than);
            }
        }
    }
}

impl Range {
    pub fn new(from: usize, to: usize) -> Self {
        assert!(to > from);
        Self(from, to)
    }

    pub fn from(&self) -> usize {
        *&self.0
    }

    pub fn to(&self) -> usize {
        *&self.1
    }

    pub fn update_less_than(&mut self, less_than: usize) {
        self.1 = less_than;
        if self.from() > self.to() {
            panic!("broken range conditions with {less_than} with {self:?}");
        }
    }

    pub fn update_more_than(&mut self, more_than: usize) {
        self.0 = more_than + 1;
        if self.from() > self.to() {
            panic!("broken range conditions with {more_than} with {self:?}");
        }
    }

    pub fn overlaps(&self, other: Self) -> bool {
        self.from().max(other.from()) < self.to().min(other.to())
    }
}

pub trait WithinRange<T>
where
    T: RangeContains,
{
    fn within(self, r: &T) -> bool;
}

pub trait RangeContains {
    fn contains(&self, i: usize) -> bool;

    fn length(&self) -> usize;
}

impl RangeContains for Range {
    fn contains(&self, i: usize) -> bool {
        i >= self.0 && i < self.1
    }

    fn length(&self) -> usize {
        self.to() - self.from()
    }
}

impl RangeContains for InclusionRange {
    fn contains(&self, i: usize) -> bool {
        self.inclusions.iter().any(|r| r.contains(i))
    }

    fn length(&self) -> usize {
        self.inclusions.iter().map(RangeContains::length).sum()
    }
}

impl<T, R> WithinRange<R> for T
where
    T: Into<usize>,
    R: RangeContains,
{
    fn within(self, r: &R) -> bool {
        let i: usize = self.into();
        r.contains(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::range::*;

    #[test]
    fn test_range_length() {
        let range = Range::new(0, 4000);
        assert_eq!(4000, range.length());
    }

    #[test]
    fn test_range_within() {
        let range = Range::new(1, 4001);
        assert_eq!(false, 0_usize.within(&range));
        assert_eq!(true, 1_usize.within(&range));
        assert_eq!(true, 4000_usize.within(&range));
        assert_eq!(false, 4001_usize.within(&range));
    }

    #[test]
    fn test_update_lt() {
        let mut range = Range::new(1, 4001);
        range.update_less_than(10);

        assert_eq!(true, 9_usize.within(&range));
        assert_eq!(false, 10_usize.within(&range));
    }

    #[test]
    fn test_update_gt() {
        let mut range = Range::new(1, 4001);
        range.update_more_than(10);

        assert_eq!(false, 10_usize.within(&range));
        assert_eq!(true, 11_usize.within(&range));
    }

    #[test]
    fn test_exclusion_range() {
        let mut r = InclusionRange::new(1, 4001);
        r.update_less_than(1416);
        r.update_more_than(2662);

        println!("{r:?}");

        assert_eq!(true, 1415_usize.within(&r));
        assert_eq!(false, 1416_usize.within(&r));

        assert_eq!(false, 2662_usize.within(&r));
        assert_eq!(true, 2663_usize.within(&r));
    }

    #[test]
    fn test_range_overlaps() {
        let a = Range::new(5, 10);

        assert_eq!(true, a.overlaps(a), "iteself");
        assert_eq!(true, a.overlaps(Range::new(9, 11)), "overlaps end");
        assert_eq!(true, a.overlaps(Range::new(0, 11)), "overlaps all");
        assert_eq!(true, a.overlaps(Range::new(6, 7)), "overlaps within");

        assert_eq!(false, a.overlaps(Range::new(11, 12)), "completely outside");
        assert_eq!(false, a.overlaps(Range::new(10, 11)), "touches upper bound");
    }
}
