#[derive(Clone, Debug)]
struct Card {
    win: Vec<usize>,
    hand: Vec<usize>,
}

fn parse_numbers(raw: &str) -> Vec<usize> {
    raw.split(' ')
        .filter(|v| !str::is_empty(v))
        .map(|v| v.parse().unwrap())
        .collect()
}

impl Card {
    fn parse(line: &str) -> Self {
        let (_, line) = line.split_once(':').unwrap();

        let (win_nums, hand_nums) = line.split_once('|').unwrap();
        let win = parse_numbers(win_nums);
        let hand = parse_numbers(hand_nums);

        Self { win, hand }
    }

    fn wins(&self) -> usize {
        self.hand.iter().filter(|v| self.win.contains(v)).count()
    }

    fn points(&self) -> usize {
        match self.wins() {
            0 => 0,
            x => (2usize).pow(x as u32 - 1),
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let cards: Vec<_> = input.split('\n').map(Card::parse).collect();

    let p1: usize = cards.iter().map(|c| c.points()).sum();
    println!("Part 1 Solution: {p1}");

    let mut stacks = vec![1usize; cards.len()];

    for idx in 0..stacks.len() {
        let card = &cards[idx];
        let wins = card.wins();
        for i in idx + 1..idx + 1 + wins {
            stacks[i] += stacks[idx];
        }
    }

    let p2: usize = stacks.iter().sum();
    println!("Part 2 Solution: {p2}");
}
