use std::collections::VecDeque;

pub fn parse_seeds(s: &str) -> Vec<u64> {
    s.split(':')
        .skip(1)
        .next()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

pub struct Mapping {
    rules: Vec<MappingItem>,
}

impl Mapping {
    pub fn parse(block: &[&str]) -> Self {
        Mapping {
            rules: block
                .iter()
                .skip(1)
                .map(|s| MappingItem::new_with_str(s))
                .collect(),
        }
    }

    pub fn convert(&self, orig: u64) -> u64 {
        self.rules
            .iter()
            .map(|mapping_item| mapping_item.try_convert(orig))
            .find(|dest| dest.is_some())
            .flatten()
            .unwrap_or(orig)
    }

    pub fn convert_range(&self, orig: SeedRange) -> Vec<SeedRange> {
        let mut deque = VecDeque::new();
        let mut result = Vec::new();
        deque.push_back(orig);

        'seed_queue: while !deque.is_empty() {
            let seedrange = deque.pop_front().unwrap();
            for rule in &self.rules {
                match rule.try_convert_range(seedrange) {
                    (Some(dest), None) => {
                        result.push(dest);
                        continue 'seed_queue;
                    }
                    (Some(dest), Some(remained)) => {
                        result.push(dest);
                        deque.push_back(remained);
                        continue 'seed_queue;
                    }
                    (None, _) => continue,
                }
            }
            result.push(seedrange);
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
pub struct SeedRange {
    pub begin: u64,
    pub length: u64,
}

impl SeedRange {
    pub fn new(begin: u64, length: u64) -> Self {
        Self { begin, length }
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.begin.cmp(&other.begin))
    }
}

struct MappingItem {
    begin: u64,
    end: u64,
    delta: i64,
}

impl MappingItem {
    fn new_with_str(s: &str) -> Self {
        let numbers = s
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|part| part.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        MappingItem {
            begin: numbers[1],
            end: numbers[1] + numbers[2],
            delta: (numbers[0] as i64) - (numbers[1] as i64),
        }
    }

    fn is_in_range(&self, pos: u64) -> bool {
        pos >= self.begin && pos < self.end
    }

    fn try_convert(&self, src: u64) -> Option<u64> {
        if self.is_in_range(src) {
            Some((src as i64 + self.delta) as u64)
        } else {
            None
        }
    }

    fn try_convert_range(&self, src: SeedRange) -> (Option<SeedRange>, Option<SeedRange>) {
        match (
            self.is_in_range(src.begin),
            self.is_in_range(src.begin + src.length - 1),
        ) {
            (false, false) => (None, None),
            (true, true) => {
                let dest = SeedRange::new((src.begin as i64 + self.delta) as u64, src.length);
                (Some(dest), None)
            }
            (true, false) => {
                let dest =
                    SeedRange::new((src.begin as i64 + self.delta) as u64, self.end - src.begin);
                let remained = SeedRange::new(src.begin + dest.length, src.length - dest.length);
                (Some(dest), Some(remained))
            }
            (false, true) => {
                let remained = SeedRange::new(src.begin, self.begin - src.begin);
                let dest = SeedRange::new(
                    (self.begin as i64 + self.delta) as u64,
                    src.length - remained.length,
                );
                (Some(dest), Some(remained))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    fn create_single_rule_mapping() -> Mapping {
        Mapping {
            rules: vec![MappingItem::new_with_str("5 10 10")],
        }
    }

    #[test]
    fn out_of_range() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(1, 5)),
            vec![SeedRange::new(1, 5)]
        );
    }

    #[test]
    fn out_of_range_but_connected() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(5, 5)),
            vec![SeedRange::new(5, 5)]
        );
    }

    #[test]
    fn tail_intersection() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(5, 6)),
            vec![SeedRange::new(5, 1), SeedRange::new(5, 5)]
        );
    
    }

    #[test]
    fn within_range() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(10, 5)),
            vec![SeedRange::new(5, 5)]
        );
    }

    #[test]
    fn head_intersection() {
        assert_eq!(
            create_single_rule_mapping().convert_range(SeedRange::new(17, 10)),
            vec![SeedRange::new(12, 3), SeedRange::new(20, 7)]
        );
    }
}
