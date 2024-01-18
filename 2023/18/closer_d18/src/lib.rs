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
            "U" => Ok(Self::Up),
            "R" => Ok(Self::Right),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "error",
            )),
        }
    }
}

pub fn parse_data(input: &str) -> (Vec<Dir>, Vec<i64>, Vec<u32>) {
    let (d, (s, c)): (Vec<_>, (Vec<_>, Vec<_>)) = input
        .lines()
        .map(|line| {
            let v: [&str; 3] = line
                .split(' ')
                .collect::<Vec<_>>()
                .as_slice()
                .try_into()
                .unwrap();
            let dir = v[0].parse::<Dir>().unwrap();
            let steps = v[1].parse::<i64>().unwrap();
            let color =
                u32::from_str_radix(v[2].trim_start_matches("(#").trim_end_matches(")"), 16)
                    .unwrap();
            (dir, (steps, color))
        })
        .unzip();
    (d, s, c)
}
