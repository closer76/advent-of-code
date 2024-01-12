pub fn solve(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let springs = parts
                .next()
                .unwrap()
                .chars()
                // For each springs list, attach a '.' at tail. This make the `can_fit()`
                // algorithm more generic.
                .chain(['.'].into_iter())
                .collect::<Vec<_>>();
            let sequences = parts
                .next()
                .unwrap()
                .split(',')
                .map(|token| token.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            solve_line(&springs, &sequences) as u64
        })
        .sum()
}

fn solve_line(springs: &[char], sequences: &[usize]) -> usize {
    if sequences.is_empty() {
        if springs.contains(&'#') {
            0
        } else {
            1
        }
    } else {
        if let Some((start, end)) = find_test_range(springs) {
            let length = sequences[0];
            let rests = &sequences[1..];
            (start..=end)
                .filter(|idx| can_fit(&springs[*idx..], length))
                .map(|idx| solve_line(&springs[(idx + length + 1)..], rests))
                .sum()
        } else {
            0
        }
    }
}

fn find_test_range(springs: &[char]) -> Option<(usize, usize)> {
    let start = springs
        .iter()
        .enumerate()
        .find(|(_, c)| **c != '.')
        .map(|(idx, _)| idx);
    let end = start
        .map(|s| {
            springs
                .iter()
                .skip(s)
                .enumerate()
                .find(|(_, c)| **c == '#')
                .map(|(idx, _)| idx + s)
        })
        .flatten();

    match (start, end) {
        (Some(s), Some(e)) => Some((s, e)),
        (Some(s), None) => Some((s, springs.len() - 1)),
        _ => None,
    }
}

fn can_fit(cs: &[char], length: usize) -> bool {
    cs.len() > length && cs.iter().take(length).map(|&c| c != '.').all(|b| b) && cs[length] != '#'
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_range() {
        let input = [
            "???.###",
            ".??..??...?##.",
            "?#?#?#?#?#?#?#?",
            "????.#...#...",
            "????.######..#####.",
            "?###????????",
            "....???#....",
        ]
        .into_iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

        assert_eq!(find_test_range(&input[0]), Some((0, 4)));
        assert_eq!(find_test_range(&input[1]), Some((1, 11)));
        assert_eq!(find_test_range(&input[2]), Some((0, 1)));
        assert_eq!(find_test_range(&input[3]), Some((0, 5)));
        assert_eq!(find_test_range(&input[4]), Some((0, 5)));
        assert_eq!(find_test_range(&input[5]), Some((0, 1)));
        assert_eq!(find_test_range(&input[6]), Some((4, 7)));
    }

    #[test]
    fn test_can_fit() {
        let cs: Vec<_> = "?###????????".chars().collect();
        assert_eq!(can_fit(&cs[0..], 3), false);
        assert_eq!(can_fit(&cs[1..], 3), true);
    }

    #[test]
    fn test_lines() {
        let springs_vec = [
            "???.###",
            ".??..??...?##.",
            "?#?#?#?#?#?#?#?",
            "????.#...#...",
            "????.######..#####.",
            "?###????????",
        ]
        .into_iter()
        .map(|s| s.chars().chain(['.'].into_iter()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

        let sequences_vec = vec![
            vec![1, 1, 3],
            vec![1, 1, 3],
            vec![1, 3, 1, 6],
            vec![4, 1, 1],
            vec![1, 6, 5],
            vec![3, 2, 1],
        ];

        assert_eq!(solve_line(&springs_vec[0], &sequences_vec[0]), 1);
        assert_eq!(solve_line(&springs_vec[1], &sequences_vec[1]), 4);
        assert_eq!(solve_line(&springs_vec[2], &sequences_vec[2]), 1);
        assert_eq!(solve_line(&springs_vec[3], &sequences_vec[3]), 1);
        assert_eq!(solve_line(&springs_vec[4], &sequences_vec[4]), 4);
        assert_eq!(solve_line(&springs_vec[5], &sequences_vec[5]), 10);
    }
}
