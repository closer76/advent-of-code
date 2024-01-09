use closer_d10::*;

pub fn solve(input: &str) -> i32 {
    let (mut grids, start_y, start_x) = build_grids(input);

    let max = (grids.len(), grids[0].len());

    let mut x = start_x;
    let mut y = start_y;
    let mut dir = grids[y][x].neighbors[0];

    let mut cw = 0;
    let mut ccw = 0;

    loop {
        (y, x) = dir.from(y, x, max).unwrap();

        if grids[y][x].is_start {
            break;
        }

        let new_dir = grids[y][x].neighbors.clone().into_iter().find(|d| *d != dir.invert()).unwrap();
        println!("{dir:?} -> {new_dir:?}");
        match (dir, new_dir) {
            (Direction::Down, Direction::Right) |
            (Direction::Right, Direction::Up) |
            (Direction::Up, Direction::Left) |
            (Direction::Left, Direction::Down)=> { ccw += 1; }
            (Direction::Down, Direction::Left) |
            (Direction::Left, Direction::Up) |
            (Direction::Up, Direction::Right) |
            (Direction::Right, Direction::Down)=> { cw += 1; }
            _ => ()
        };
        dir = new_dir;
    }

    println!("ccw: {}, cw: {}", ccw, cw);

    // while !queue.is_empty() {
    //     let (y, x) = queue.pop_front().unwrap();
    //     grids[y][x].visited = true;
    //     grids[y][x].neighbors.clone().into_iter().for_each(|dir| {
    //         if let Some((ny, nx)) = dir.from(y, x, (height, width)) {
    //             if grids[ny][nx].visited {
    //                 grids[y][x].distance = grids[ny][nx].distance + 1;
    //             } else {
    //                 queue.push_back((ny, nx))
    //             }
    //         }
    //     })
    // }

    0
}
