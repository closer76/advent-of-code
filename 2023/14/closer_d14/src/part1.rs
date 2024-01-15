pub fn solve(lines: &Vec<&str>) -> u64 {
    let height = lines.len();
    let width = lines[0].chars().count();

    let mut weight = vec![(height + 1) as u64; width];

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'O' => {
                        weight[x] -= 1;
                        weight[x]
                    }
                    '#' => {
                        weight[x] = (height - y) as u64;
                        0
                    }
                    _ => 0,
                })
                .sum::<u64>()
        })
        .sum()
}
