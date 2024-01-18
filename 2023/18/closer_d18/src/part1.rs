use closer_d18::*;

fn go(orig_y: i64, orig_x: i64, dir: Dir, count: i64) -> (i64, i64) {
    match dir {
        Dir::Right => (orig_y, orig_x + count),
        Dir::Up => (orig_y - count, orig_x),
        Dir::Left => (orig_y, orig_x - count),
        Dir::Down => (orig_y + count, orig_x),
    }
}

pub fn solve(input: &str) -> i64 {
    let (dirs, counts, _) = parse_data(input);

    let mut current = (0, 0);
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);

    dirs.iter().zip(counts.iter()).for_each(|(&d, &c)| {
        current = go(current.0, current.1, d, c);
        min = (min.0.min(current.0), min.1.min(current.1));
        max = (max.0.max(current.0), max.1.max(current.1));
    });

    let mut grids = vec![vec![false; (max.1 - min.1 + 3) as usize]; (max.0 - min.0 + 3) as usize];
    let mut current = (-min.0, -min.1);
    dirs.iter().zip(counts.iter()).for_each(|(&d, &c)| {
        for _ in 0..c {
            current = go(current.0, current.1, d, 1);
            grids[(current.0 + 1) as usize][(current.1 + 1) as usize] = true;
        }
    });

    grids
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .fold((0, false, false), |(acc, up, down), (x, &grid)| {
                    if grid {
                        (acc + 1, up ^ grids[y - 1][x], down ^ grids[y + 1][x])
                    } else {
                        if up && down {
                            (acc + 1, up, down)
                        } else {
                            (acc, up, down)
                        }
                    }
                })
                .0
        })
        .sum::<i64>()
}
