use lib::*;

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    n: usize,
    color: String,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let dir = input[..1].into();

        let (n, color) = input[2..].split_once(' ').unwrap();
        let n = n.parse().unwrap();
        let color = color[2..color.len() - 1].into();

        Self { dir, n, color }
    }

    fn transform_from_colors(&self) -> Self {
        let n: usize = usize::from_str_radix(&self.color[0..5], 16).unwrap();
        let dir = match &self.color.chars().nth(5).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            x => panic!("invalid direction: {x}"),
        };

        Self {
            dir,
            n,
            color: self.color.clone(),
        }
    }
}

fn size(instructions: &[Instruction]) -> usize {
    let mut pos = Pos::default();
    let mut points = vec![pos];

    for i in instructions {
        pos += i.dir * i.n;
        points.push(pos);
    }

    let n = points.len();
    let mut area = 0;
    let mut perimeter = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += points[i].x * points[j].y;
        area -= points[i].y * points[j].x;
        perimeter += (points[i].x - points[j].x).abs() + (points[i].y - points[j].y).abs();
    }

    ((area.abs() as f64 / 2.0) + perimeter as f64 / 2.0) as usize + 1
}

fn main() {
    let input: String = lib::read_input!();

    let instructions: Vec<_> = input.split('\n').map(Instruction::parse).collect();
    p1!(size(&instructions));

    let instructions: Vec<_> = instructions
        .iter()
        .map(|i| i.transform_from_colors())
        .collect();
    p2!(size(&instructions));
}
