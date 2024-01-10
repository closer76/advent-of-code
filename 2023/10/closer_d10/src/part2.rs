use closer_d10::*;

pub fn solve(input: &str) -> i32 {
    let (mut grids, start_y, start_x) = build_grids(input);

    let max = (grids.len(), grids[0].len());


    let mut x = start_x;
    let mut y = start_y;
    let mut dir = grids[y][x].neighbors[0];
    let mut cw = 0;
    let mut ccw = 0;

    grids[y][x].visited = true;
    loop {
        (y, x) = dir.from(y, x, max).unwrap();

        if grids[y][x].is_start {
            break;
        }

        grids[y][x].visited = true;

        let new_dir = grids[y][x]
            .neighbors
            .clone()
            .into_iter()
            .find(|d| *d != dir.turn_180())
            .unwrap();
        if new_dir == dir.turn_90(true) {
            ccw += 1;
        } else if new_dir == dir.turn_90(false) {
            cw += 1;
        }
        dir = new_dir;
    }

    let is_ccw = ccw > cw;

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = grids[y][x].neighbors[0].turn_180();

    loop {
        let new_dir = grids[y][x]
            .neighbors
            .clone()
            .into_iter()
            .find(|d| *d != dir.turn_180())
            .unwrap();
        [dir, new_dir].iter().for_each(|d| {
            if let Some((ny, nx)) = d.turn_90(!is_ccw).from(y, x, max) {
                if !grids[ny][nx].visited {
                    grids[ny][nx].is_in = true;
                }
            }
        });
        dir = new_dir;
        (y, x) = dir.from(y, x, max).unwrap();

        if grids[y][x].is_start {
            break;
        }
    }

    for y in 1..grids.len() {
        for x in 1..grids[y].len() {
            if !grids[y][x].visited
                && !grids[y][x].is_in
                && (grids[y - 1][x].is_in || grids[y][x - 1].is_in)
            {
                grids[y][x].is_in = true;
            }
        }
    }
    // println!("{}", format_grids(&grids));

    grids
        .iter()
        .map(|line| {
            line.iter()
                .map(|grid| {
                    if grid.is_in {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum()
}
