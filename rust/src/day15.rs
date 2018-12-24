const INPUT : &str = include_str!("../inputs/day15.txt");

use std::fmt;

#[derive(Debug)]
struct Fighter {
    icon: char,
    hp:   usize,
    ap:   usize,
    pos:  (usize, usize),
    alive: bool
}


struct Arena {
    grid: Vec<Vec<char>>,
    fighters: Vec<Fighter>
}

enum FightAction {
    Move    { index: usize, pos: (usize, usize) },
    TakeDmg { index: usize, amount: usize }
}

impl Fighter {
    pub fn new(icon: char, pos: (usize, usize)) -> Self {
        Fighter {
            icon: icon,
            hp:   200,
            ap:   3,
            pos:  pos,
            alive: true
        }
    }

    pub fn turn(&self, arena: &Arena) -> Option<FightAction> {
        None
    }
}

impl Arena {
    pub fn fighter_at(&self, pos: (usize, usize)) -> Option<&Fighter> {
        self.fighters.iter().find(|f| f.pos == pos)
    }

    pub fn round(&mut self) -> bool {
        self.fighters.sort_by_key(|f| f.pos);

        let actions : Vec<FightAction> = self.fighters
            .iter()
            .filter(|f| f.alive)
            .map(|f| f.turn(&self) )
            .filter(|o| o.is_some())
            .map(|o| o.unwrap())
            .collect();

        for a in actions.iter() {
            match a {
                &FightAction::TakeDmg { index, amount } => {
                    if amount >= self.fighters[index].hp {
                        self.fighters[index].hp    = 0;
                        self.fighters[index].alive = false;
                    } else {
                        self.fighters[index].hp -= amount;
                    }
                },
                &FightAction::Move { index, pos } => self.fighters[index].pos = pos
            }
        };

        actions.is_empty()
    }

    pub fn load(input: &str) -> Arena {
        let mut fighters = vec![];

        let grid = input
            .lines()
            .enumerate()
            .map(|(row, line)|{
                line
                    .chars()
                    .enumerate()
                    .map(|(col, ch)|{
                        match ch {
                            'E' | 'G' => {
                                fighters.push(Fighter::new(ch, (row, col)));
                                '.'
                            },
                            '#' | '.' => ch,
                            _ => panic!("Bad input token")
                        }
                    })
                    .collect()
            })
            .collect();

        Arena {
            grid:     grid,
            fighters: fighters
        }
    }
}

impl fmt::Display for Arena {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (row_no, row) in self.grid.iter().enumerate() {
            for (col_no, ch) in row.iter().enumerate() {
                let c = match self.fighter_at((col_no, row_no)) {
                    Some(ft) => &ft.icon,
                    None     => ch
                };
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        };

        Ok(())
    }

}


mod tests {
    use super::*;

    #[test]
    fn day15_p1() {
        let mut a = Arena::load(INPUT);
        a.round();
        println!("{}", a);

        println!("{:?}", a.fighters);
    }

}
