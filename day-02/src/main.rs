#[derive(Default, Debug)]
struct Cubes {
    red: u32,
    blue: u32,
    green: u32,
}

impl Cubes {
    fn parse(raw: &str) -> Self {
        let mut round = Self::default();

        for cube in raw.split(',').map(str::trim) {
            let (n, color) = cube.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "blue" => round.blue = n,
                "green" => round.green = n,
                "red" => round.red = n,
                _ => panic!("unexpected color: {color}"),
            }
        }

        round
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Cubes>,
}

impl Game {
    fn parse(line: &str) -> Self {
        let idx = line.chars().position(|c| c == ':').unwrap();
        let id = line[5..idx].parse().unwrap();

        let rounds = line[idx + 1..].split(';').map(Cubes::parse).collect();

        Self { id, rounds }
    }

    fn max_cubes(&self) -> Cubes {
        Cubes {
            red: self.rounds.iter().map(|r| r.red).max().unwrap(),
            green: self.rounds.iter().map(|r| r.green).max().unwrap(),
            blue: self.rounds.iter().map(|r| r.blue).max().unwrap(),
        }
    }

    fn is_possible(&self, max: &Cubes) -> bool {
        self.rounds
            .iter()
            .all(|r| r.blue <= max.blue && r.red <= max.red && r.green <= max.green)
    }
}

fn main() {
    let input: String = lib::read_input!();

    let games: Vec<_> = input.split('\n').map(Game::parse).collect();

    let max = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let d1: u32 = games
        .iter()
        .filter(|g| g.is_possible(&max))
        .map(|g| g.id)
        .sum();

    println!("Day 1 Solution: {d1}");

    let d2: u32 = games.iter().map(Game::max_cubes).map(|c| c.power()).sum();

    println!("Day 2 Solution: {d2}");
}
