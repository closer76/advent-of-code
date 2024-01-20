use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::iter;
use std::slice::Chunks;
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

    let mut levels = Vec::new();
    let mut cur_y = i64::MAX;
    let mut row = Vec::new();
    for v in points.into_sorted_vec().chunks(2) {
        let (y0, x0) = v[0];
        let (y1, x1) = v[1];
        assert_eq!(y0, y1);
        if y0 != cur_y {
            if !row.is_empty() {
                levels.push((cur_y, row));
                row = Vec::new();
            }
            cur_y = y0;
        }
        row.push((x0, x1));
    }
    levels.push((cur_y, row));

    for row in &levels {
        println!("{:?}", row);
    }

    let mut last_y = i64::MIN;

    let v = levels
        .iter()
        .fold((0, vec![]), |(area, segments), row| {
            let cur_y = row.0;
            let inner_area = if segments.is_empty() {
                0
            } else {
                eval_segments(&segments) * (cur_y - last_y - 1)
            };
            last_y = cur_y;

            let mut result: Vec<(i64, i64)> = vec![];
            let mut iter_prev = segments.iter();
            let mut iter_cur = row.1.iter();
            let mut seg_prev: Option<&(i64, i64)> = iter_prev.next();
            let mut seg_cur = iter_cur.next();

            loop {
                let mut new_seg: Vec<(i64, i64)> = vec![];
                match (seg_prev, seg_cur) {
                    (Some(v_prev), Some(v_cur)) => {
                        if v_prev.0 == v_cur.0 {
                            if v_prev.1 != v_cur.1 {
                                new_seg.push((v_prev.1.min(v_cur.1), v_prev.1.max(v_cur.1)));
                            }
                            seg_prev = iter_prev.next();
                            seg_cur = iter_cur.next();
                        } else if v_prev.1 == v_cur.1 {
                            new_seg.push((v_prev.0, v_cur.0));
                            seg_prev = iter_prev.next();
                            seg_cur = iter_cur.next();
                        } else if v_prev.1 == v_cur.0 || v_prev.0 == v_cur.1 {
                            new_seg.push((v_prev.0.min(v_cur.0), v_prev.1.max(v_cur.1)));
                            seg_prev = iter_prev.next();
                            seg_cur = iter_cur.next();
                        } else if v_prev.1 < v_cur.0 {
                            new_seg.push((v_prev.0, v_prev.1));
                            seg_prev = iter_prev.next();
                        } else if v_cur.1 < v_prev.0 {
                            new_seg.push(*v_cur);
                            seg_cur = iter_cur.next()
                        } else if v_prev.0 < v_cur.0 && v_prev.1 > v_cur.1 {
                            new_seg.push((v_prev.0, v_cur.0));
                            new_seg.push((v_cur.1, v_prev.1));
                            seg_prev = iter_prev.next();
                            seg_cur = iter_cur.next();
                        } else {
                            new_seg.push(*v_cur);
                            seg_prev = iter_prev.next();
                            seg_cur = iter_cur.next();
                        }
                    }
                    (Some(v1), None) => {
                        new_seg.push(*v1);
                        seg_prev = iter_prev.next()
                    }
                    (None, Some(v2)) => {
                        new_seg.push(*v2);
                        seg_cur = iter_cur.next()
                    }
                    _ => break,
                }

                for seg in new_seg {
                    match result.last_mut() {
                        Some((_, x1)) if *x1 >= seg.0 => {
                            *x1 = seg.1.max(*x1);
                        }
                        _ => {
                            result.push((seg.0, seg.1));
                        }
                    }
                }
            }

            println!("{last_y} - {:?}", result);

            let line_area = eval_segments(&merge_segments(&segments, &result));
            println!("{inner_area} + {line_area}");

            (area + inner_area + line_area, result)
        })
        .0;

    v
}

fn merge_segments(segs0: &Vec<(i64, i64)>, segs1: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut result = vec![];
    let mut iter0 = segs0.iter();
    let mut iter1 = segs1.iter();
    let mut seg0 = iter0.next();
    let mut seg1 = iter1.next();

    while seg0.is_some() || seg1.is_some() {
        match (seg0, seg1) {
            (Some(v0), Some(v1)) => {
                if v0.0 == v1.0 || v0.1 == v1.1 || v0.0 == v1.1 || v0.1 == v1.0 {
                    result.push((v0.0.min(v1.0), v0.1.max(v1.1)));
                    seg0 = iter0.next();
                    seg1 = iter1.next();
                } else if v0.0 < v1.0 {
                    result.push(*v0);
                    seg0 = iter0.next();
                } else {
                    result.push(*v1);
                    seg1 = iter1.next();
                }
            }
            (Some(v0), None) => {
                result.push(*v0);
                seg0 = iter0.next();
            }
            (None, Some(v1)) => {
                result.push(*v1);
                seg1 = iter1.next();
            }
            _ => panic!("Impossible!"),
        }
    }

    result
}

fn eval_segments(segs: &Vec<(i64, i64)>) -> i64 {
    segs.iter().fold(0, |acc, (x0, x1)| acc + (x1 - x0 + 1))
}
