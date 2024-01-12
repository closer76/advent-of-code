use std::cell::RefCell;
use std::{collections::HashMap, iter};

pub fn solve(input: &Vec<&str>) -> u64 {
    input
        .iter()
        .map(|line| {
            let mut parts = line.split(' ');
            let springs_vec = parts.next().unwrap().chars().collect::<Vec<_>>();
            let springs = iter::repeat(springs_vec)
                .take(5)
                .collect::<Vec<_>>()
                .join(&'?')
                .into_iter()
                .chain(['.'].into_iter())
                .collect::<Vec<_>>();

            let sequences_vec = parts
                .next()
                .unwrap()
                .split(',')
                .map(|token| token.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let sequences = iter::repeat(sequences_vec)
                .take(5)
                .collect::<Vec<_>>()
                .join(&[][..]);

            let mut memory = RefCell::new(HashMap::new());
            solve_line_caches(&springs, &sequences, &mut memory) as u64
        })
        .sum()
}

pub fn solve_line_caches<'a>(
    springs: &'a [char],
    sequences: &'a [usize],
    memory: &RefCell<HashMap<(Vec<char>, Vec<usize>), usize>>,
) -> usize {
    if sequences.is_empty() {
        if springs.contains(&'#') {
            0
        } else {
            1
        }
    } else {
        let end = springs
            .iter()
            .enumerate()
            .find(|(_, c)| **c == '#')
            .unwrap_or((springs.len(), &'_'))
            .0;

        let mut sum = 0;

        (0..=end)
            .filter(|idx| closer_d12::can_fit(&springs[*idx..], sequences[0]))
            .for_each(|idx| {
                let new_springs = springs[(idx + sequences[0] + 1)..].to_vec();
                let new_sequence = sequences[1..].to_vec();
                let c = memory
                    .borrow()
                    .get(&(new_springs.clone(), new_sequence.clone()))
                    .map(|v| *v);
                let x = match c {
                    Some(value) => value,
                    None => {
                        let value = solve_line_caches(
                            &springs[(idx + sequences[0] + 1)..],
                            &sequences[1..],
                            memory,
                        );
                        memory
                            .borrow_mut()
                            .insert((new_springs, new_sequence), value);
                        value
                    }
                };
                sum += x;
            });

        sum
    }
}
