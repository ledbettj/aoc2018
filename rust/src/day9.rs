
const INPUT : &str = include_str!("../inputs/day9.txt");


#[derive(Debug)]
struct Game {
    players:        usize,
    last_marble:    usize,
    scores:         Vec<usize>,
    marbles:        Vec<usize>,
    current:        Option<usize>
}

impl Game {
    pub fn new(players: usize, last_marble: usize) -> Game {
        Game {
            players:     players,
            last_marble: last_marble,
            scores:      vec![0; players],
            marbles:     Vec::with_capacity(last_marble),
            current:     None
        }
    }

    pub fn high_score(&self) -> usize {
        *self.scores.iter().max().unwrap()
    }

    pub fn tick(&mut self, player: usize, marble: usize) {
        match self.current {
            Some(n) => {
                if marble % 23 == 0 {
                    self.scores[player] += marble;

                    let remove_at = if n >= 7 {
                        n - 7
                    } else {
                        self.marbles.len() - (7 - n)
                    };

                    self.scores[player] += self.marbles.remove(remove_at);
                    self.current = Some(remove_at % self.marbles.len());
                } else {
                    let next_index = ((n + 1) % self.marbles.len()) + 1;
                    self.marbles.insert(next_index, marble);
                    self.current = Some(next_index);
                }
            },
            None => {
                /* empty setup, better be zero */
                self.marbles.push(marble);
                self.current = Some(0);
            }
        }

    }
}


mod tests {
    use super::*;

    #[test]
    fn day9_p1() {
        let players = 405;
        let last = 71700;

        let mut g = Game::new(players, last);

        for n in (0..(last + 1)) {
            g.tick(n % players, n);
        }
        assert_eq!(g.high_score(), 428690);
    }

    #[test]
    fn day9_p2() {
        let players = 405;
        let last = 71700 * 100;

        let mut g = Game::new(players, last);

        for n in (0..(last + 1)) {
            g.tick(n % players, n);
        }
        assert_eq!(g.high_score(), 428690);
    }

}
