enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    fn pick(&self, against: &Shape) -> Shape {
        match self {
            Outcome::Win => match against {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Draw => against.clone(),
            Outcome::Loss => match against {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        }
    }
}

impl From<&str> for Outcome {
    fn from(c: &str) -> Self {
        match c {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("That's not a result!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn against(&self, other: &Self) -> Outcome {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }

    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl From<&str> for Shape {
    fn from(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("That's not a shape!"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    ours: Shape,
    theirs: Shape,
}

impl Game {
    fn play(&self) -> i32 {
        let outcome = self.ours.against(&self.theirs);
        outcome.score() + self.ours.score()
    }

    fn from_pick(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        let theirs = tokens.next().expect("Not theirs");
        let ours = tokens.next().expect("Not ours");

        Self {
            ours: ours.into(),
            theirs: theirs.into(),
        }
    }

    fn from_outcome(s: &str) -> Self {
        let mut tokens = s.split_whitespace();
        let theirs: Shape = tokens.next().expect("Not theirs").into();
        let outcome: Outcome = tokens.next().expect("Not ours").into();

        Self {
            ours: outcome.pick(&theirs),
            theirs,
        }
    }
}

#[derive(Debug)]
struct StrategyGuide(Vec<Game>);

impl StrategyGuide {
    fn new_pick(input: &str) -> Self {
        Self(input.lines().map(|line| Game::from_pick(line)).collect())
    }

    fn new_outcome(input: &str) -> Self {
        Self(input.lines().map(|line| Game::from_outcome(line)).collect())
    }

    fn score(&self) -> i32 {
        self.0.iter().fold(0, |score, game| score + game.play())
    }
}

fn main() {
    let input = std::fs::read_to_string("day2.prod").expect("Gimme the input");

    let guide = StrategyGuide::new_pick(input.as_str());
    let result = guide.score();

    println!("Pick Score: {result}");

    let guide = StrategyGuide::new_outcome(input.as_str());
    let result = guide.score();

    println!("Outcome Score: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_GAME: &str = r#"A Y
B X
C Z"#;

    #[test]
    fn test_game() {
        let guide = StrategyGuide::new_pick(TEST_GAME);
        assert_eq!(guide.score(), 15);
    }

    #[test]
    fn test_outcome() {
        let guide = StrategyGuide::new_outcome(TEST_GAME);
        assert_eq!(guide.score(), 12);
    }
}
