use std::collections::BinaryHeap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Dir {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" | "0" => Ok(Self::Right),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "U" | "3" => Ok(Self::Up),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "error",
            )),
        }
    }
}

pub fn parse_data(input: &str) -> (Vec<Dir>, Vec<i64>, Vec<Dir>, Vec<i64>) {
    let (d1, (s1, (d2, s2))): (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))) = input
        .lines()
        .map(|line| {
            let v: [&str; 3] = line
                .split(' ')
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap();
            let dir_1 = v[0].parse::<Dir>().unwrap();
            let steps_1 = v[1].parse::<i64>().unwrap();
            let dir_2 = (&v[2][7..8]).parse::<Dir>().unwrap();
            let steps_2 = i64::from_str_radix(&v[2][2..7], 16).unwrap();
            (dir_1, (steps_1, (dir_2, steps_2)))
        })
        .unzip();
    (d1, s1, d2, s2)
}

fn go(orig_y: i64, orig_x: i64, dir: Dir, steps: i64) -> (i64, i64) {
    match dir {
        Dir::Right => (orig_y, orig_x + steps),
        Dir::Up => (orig_y - steps, orig_x),
        Dir::Left => (orig_y, orig_x - steps),
        Dir::Down => (orig_y + steps, orig_x),
    }
}

pub fn eval_slow(dirs: &Vec<Dir>, steps: &Vec<i64>) -> i64 {
    let mut current = (0, 0);
    let mut min = (i64::MAX, i64::MAX);
    let mut max = (i64::MIN, i64::MIN);

    dirs.iter().zip(steps.iter()).for_each(|(&d, &c)| {
        current = go(current.0, current.1, d, c);
        min = (min.0.min(current.0), min.1.min(current.1));
        max = (max.0.max(current.0), max.1.max(current.1));
    });

    let mut grids = vec![vec![false; (max.1 - min.1 + 3) as usize]; (max.0 - min.0 + 3) as usize];
    let mut current = (-min.0, -min.1);
    dirs.iter().zip(steps.iter()).for_each(|(&d, &c)| {
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

pub fn eval_fast(dirs: &Vec<Dir>, steps: &Vec<i64>) -> i64 {
    let mut points = BinaryHeap::new();
    let mut pos = (0_i64, 0_i64);

    dirs.iter().zip(steps.iter()).for_each(|(&d, &s)| {
        pos = go(pos.0, pos.1, d, s);
        points.push(pos);
    });

    let mut levels = vec![(i32::MIN as i64, vec![])];
    let mut cur_y = i64::MIN;
    let mut row = Vec::new();
    for v in points.into_sorted_vec() {
        if v.0 != cur_y {
            if !row.is_empty() {
                levels.push((cur_y, row));
                row = Vec::new();
            }
            cur_y = v.0;
        }
        row.push(v.1);
    }
    levels.push((cur_y, row));

    levels
        .windows(2)
        .fold((0, vec![]), |(area, segments), row_pair| {
            let (last_y, _) = row_pair[0];
            let (cur_y, row) = &row_pair[1];

            let new_segments = proceed_row(&segments, row);
            // println!("{}:: {:?} + {:?} = {:?}", row.0, segments, row.1, new_segments);

            let inner_area = eval_segments(&segments) * (cur_y - last_y - 1);
            let line_area = eval_segments(&merge_segments(&segments, &new_segments));
            // println!("{inner_area} + {line_area}");

            (area + inner_area + line_area, new_segments)
        })
        .0
}

fn proceed_row(base: &Vec<i64>, added: &Vec<i64>) -> Vec<i64> {
    let mut base_iter = base.iter();
    let mut added_iter = added.iter();
    let mut result = vec![];

    let mut base_point = base_iter.next();
    let mut added_point = added_iter.next();

    while base_point.is_some() || added_point.is_some() {
        match (base_point, added_point) {
            (Some(base_v), Some(added_v)) => {
                if base_v == added_v {
                    base_point = base_iter.next();
                    added_point = added_iter.next();
                } else if base_v < added_v {
                    result.push(*base_v);
                    base_point = base_iter.next();
                } else {
                    result.push(*added_v);
                    added_point = added_iter.next();
                }
            }
            (Some(base_v), None) => {
                result.push(*base_v);
                base_point = base_iter.next();
            }
            (None, Some(added_v)) => {
                result.push(*added_v);
                added_point = added_iter.next();
            }
            _ => panic!("Both are None??"),
        }
    }

    result
}

#[test]
fn simple_tests() {
    assert_eq!(proceed_row(&vec![], &vec![0, 6]), vec![0, 6]);
    assert_eq!(proceed_row(&vec![0, 6], &vec![0, 2]), vec![2, 6]);
    assert_eq!(proceed_row(&vec![2, 6], &vec![0, 2, 4, 6]), vec![0, 4]);
    assert_eq!(proceed_row(&vec![0, 4], &vec![0, 1, 4, 6]), vec![1, 6]);
    assert_eq!(proceed_row(&vec![1, 6], &vec![1, 6]), vec![]);
}

fn merge_segments(segs0: &Vec<i64>, segs1: &Vec<i64>) -> Vec<i64> {
    let mut iter0 = segs0.iter().zip([true, false].into_iter().cycle());
    let mut iter1 = segs1.iter().zip([true, false].into_iter().cycle());
    let mut result = vec![];

    let mut in0 = false;
    let mut in1 = false;

    let mut is_in = false;

    let mut point0 = iter0.next();
    let mut point1 = iter1.next();

    while point0.is_some() || point1.is_some() {
        let cand = match (point0, point1) {
            (Some((&p0_v, p0_in)), Some((&p1_v, p1_in))) => {
                if p0_v == p1_v {
                    point0 = iter0.next();
                    point1 = iter1.next();
                    in0 = p0_in;
                    in1 = p0_in;
                    p0_v
                } else if p0_v < p1_v {
                    point0 = iter0.next();
                    in0 = p0_in;
                    p0_v
                } else {
                    point1 = iter1.next();
                    in1 = p1_in;
                    p1_v
                }
            }
            (Some((&p0_v, p0_in)), None) => {
                point0 = iter0.next();
                in0 = p0_in;
                p0_v
            }
            (None, Some((&p1_v, p1_in))) => {
                point1 = iter1.next();
                in1 = p1_in;
                p1_v
            }
            _ => panic!("Both are None??"),
        };

        let new_is_in = in0 || in1;

        if new_is_in != is_in {
            result.push(cand);
            is_in = new_is_in;
        }
    }

    result
}

#[test]
fn simple_merge_test() {
    assert_eq!(merge_segments(&vec![0, 6], &vec![2, 6]), vec![0, 6]);
    assert_eq!(merge_segments(&vec![2, 6], &vec![0, 4]), vec![0, 6]);
    assert_eq!(merge_segments(&vec![0, 4], &vec![1, 6]), vec![0, 6]);
}

fn eval_segments(segs: &Vec<i64>) -> i64 {
    segs.chunks(2)
        .fold(0, |acc, chunk| acc + (chunk[1] - chunk[0] + 1))
}
