use itertools::Itertools;
use std::collections::HashSet;

pub fn challenge_06(line: &str, seq_len: usize) -> Option<usize> {
    // Non-iterator
    let chars: Vec<_> = line.chars().collect();
    let mut ans = None;
    for i in 0..(chars.len() - seq_len) {
        let mut set = HashSet::new();
        for j in i..(i + seq_len) {
            set.insert(chars[j]);
        }
        if set.len() == seq_len {
            ans = Some(i + seq_len);
            break;
        }
    }

    // Iterator
    line.as_bytes()
        .windows(seq_len)
        .find_position(|v| v.iter().collect::<HashSet<_>>().len() == seq_len)
        .map(|(pos, _)| pos + seq_len)
}

#[test]
fn test_part1() {
    assert_eq!(challenge_06("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), Some(7));
    assert_eq!(challenge_06("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(challenge_06("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    assert_eq!(
        challenge_06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
        Some(10)
    );
    assert_eq!(
        challenge_06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4),
        Some(11)
    );
}

#[test]
fn test_part2() {
    assert_eq!(challenge_06("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
    assert_eq!(challenge_06("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
    assert_eq!(challenge_06("nppdvjthqldpwncqszvftbrmjlhg", 14), Some(23));
    assert_eq!(
        challenge_06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14),
        Some(29)
    );
    assert_eq!(
        challenge_06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14),
        Some(26)
    );
}
