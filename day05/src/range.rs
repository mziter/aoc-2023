#[derive(Debug)]
pub struct SeedMap {
    rules: Vec<MapRule>,
}

impl SeedMap {
    pub fn new(rules: Vec<MapRule>) -> SeedMap {
        SeedMap { rules }
    }

    pub fn add_rule(&mut self, new_rule: &MapRule) {
        for i in 0..self.rules.len() {
            let existing_rule = self.rules[i];
            if self.rules[i].in_overlaps_out(new_rule) {
                self.rules.retain(|r| *r != existing_rule);
                self.rules.append(&mut existing_rule.merge_in(new_rule))
            } else {
                self.rules.push(*new_rule)
            }
        }
    }

    pub fn translate(&self, input: &u64) -> u64 {
        for range in self.rules.iter() {
            if let Some(output) = range.translate(input) {
                return output;
            };
        }
        *input
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapRule {
    out_start: u64,
    in_start: u64,
    len: u64,
}

impl MapRule {
    pub fn new(out_start: u64, in_start: u64, len: u64) -> MapRule {
        MapRule {
            out_start,
            in_start,
            len,
        }
    }

    pub fn translate(&self, input: &u64) -> Option<u64> {
        if *input >= self.in_start && *input < self.in_start + self.len {
            return Some(self.out_start + (input - self.in_start));
        }
        None
    }

    pub fn merge_in(&self, r: &MapRule) -> Vec<MapRule> {
        todo!()
    }

    pub fn in_overlaps_out(&self, r: &MapRule) -> bool {
        (r.out_start >= self.in_start && r.out_start <= self.in_end())
            || (r.out_end() >= self.in_start && r.out_end() <= self.in_end())
    }

    fn in_end(&self) -> u64 {
        self.in_start + self.len - 1
    }

    fn out_end(&self) -> u64 {
        self.out_start + self.len - 1
    }
    /*
    fn contains_in(&self, n: &u64) -> bool {
        *n >= self.in_start && *n < self.in_start + self.len
    }

    fn contains_out(&self, n: &u64) -> bool {
        *n >= self.out_start && *n < self.out_start + self.len
    }
    */
}

/*
#[derive(Debug, Clone, Copy)]
struct SeedMapRule<T, U>
where
    T: RangeBounds<u64>,
    U: RangeBounds<u64>,
{
    in_range: T,
    out_range: U,
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_merge_rules() {
        let a = MapRule {
            out_start: 0,
            in_start: 15,
            len: 10,
        };
        let b = MapRule {
            out_start: 0,
            in_start: 24,
            len: 3,
        };
        */

    #[test]
    //    A       B          A       C        B
    //  len=4   len=4      len=4   len=2    len=2
    //
    //           2 1
    //   1 3     3 2                1 2
    //   2 4     4 3                2 3
    //   3 5     5 4   =>   3 5              5 4
    //   4 6                4 6
    //
    //
    //  A TRIMMED UP 2 (increase start params and decrease length by overlap)
    //  B TRIMMED DOWN 2 (reduce length by overlap)
    //  C (len equals overlap amount, in starts at A in and offset is combined
    //  offset of both a and b)
    //
    //
    fn test_in_overlaps_out() {
        let a = MapRule {
            in_start: 1,
            out_start: 3,
            len: 4,
        };
        let b = MapRule {
            in_start: 2,
            out_start: 1,
            len: 4,
        };

        let expected = [
            MapRule {
                in_start: 3,
                out_start: 5,
                len: 2,
            },
            MapRule {
                in_start: 5,
                out_start: 4,
                len: 1,
            },
            MapRule {
                in_start: 1,
                out_start: 2,
                len: 2,
            },
        ];
        assert!(b.in_overlaps_out(&a))
    }
}
