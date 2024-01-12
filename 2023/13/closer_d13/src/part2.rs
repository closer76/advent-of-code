use std::collections::VecDeque;

pub fn solve(patterns: &Vec<Vec<&str>>) -> u64 {
    patterns.iter().map(|pattern| handle_pattern(pattern)).sum()
}

fn handle_pattern(pattern: &Vec<&str>) -> u64 {
    let width = pattern[0].len();
    let height = pattern.len();
    assert!(width < 64);
    assert!(height < 64);

    let mut cols = VecDeque::from_iter(std::iter::repeat(vec!['.'; height]).take(width));
    let mut rows = VecDeque::from_iter(std::iter::repeat(vec!['.'; width]).take(height));

    pattern.iter().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            cols[x][y] = c;
            rows[y][x] = c;
        })
    });

    // println!("cols: {cols:?}");
    // println!("rows: {rows:?}");

    evaluate(rows) * 100 + evaluate(cols)
}

fn evaluate(mut src: VecDeque<Vec<char>>) -> u64 {
    let mut dest = VecDeque::new();
    while src.len() > 1 {
        dest.push_front(src.pop_front().unwrap());

        // println!("s: {:?}", pretty(&src));
        // println!("d: {:?}", pretty(&dest));
        let diff_count = src
            .iter()
            .zip(dest.iter())
            .map(|(s, d)| {
                s.iter()
                    .zip(d.iter())
                    .map(|(sc, dc)| if *sc == *dc { 0 } else { 1 })
                    .sum::<u64>()
            })
            .sum::<u64>();

        if diff_count == 1 {
            // println!("Hit!");
            return dest.len() as u64;
        }
    }
    0
}

// fn pretty(list: &VecDeque<Vec<char>>) -> Vec<u64> {
//     list.iter()
//         .map(|item| {
//             item.iter().fold(
//                 0,
//                 |acc, &c| {
//                     if c == '#' {
//                         (acc << 1) | 1
//                     } else {
//                         acc << 1
//                     }
//                 },
//             )
//         })
//         .collect::<Vec<_>>()
// }
