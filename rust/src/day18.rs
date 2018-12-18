const INPUT : &str = include_str!("../inputs/day18.txt");

use std::fmt;

#[derive(Debug,PartialEq,Clone)]
struct ConstructionArea {
    grid: Vec<Vec<char>>
}

impl ConstructionArea {
    pub fn load(input: &str) -> ConstructionArea {
        let grid = input
            .lines()
            .map(|line| line.chars().collect() )
            .collect();

        ConstructionArea { grid: grid }
    }

    pub fn next(&self) -> ConstructionArea {
        let mut result : Vec<Vec<char>> =  Vec::with_capacity(self.grid.len());

        for (row_no, row) in self.grid.iter().enumerate() {
            let mut new_row : Vec<char> = Vec::with_capacity(row.len());

            for (col_no, ch) in row.iter().enumerate() {
                let new_ch = match ch {
                    '.' => {
                        if self.count_adjacent((row_no, col_no), '|') >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    },
                    '|' => {
                        if self.count_adjacent((row_no, col_no), '#') >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    },
                    '#' => {
                        if self.count_adjacent((row_no, col_no), '#') >= 1 && self.count_adjacent((row_no, col_no), '|') >= 1  {
                            '#'
                        } else {
                            '.'
                        }
                    },
                    _   => panic!("Bad input character")
                };
                new_row.push(new_ch);
            }
            result.push(new_row);
        };

        ConstructionArea { grid: result }
    }

    fn count_adjacent(&self, pos: (usize, usize), look: char) -> usize {
        let mut rmin : isize = pos.0 as isize - 1;
        let mut cmin : isize = pos.1 as isize - 1;
        let mut rmax : isize = rmin + 3;
        let mut cmax : isize = cmin + 3;
        let mut count = 0;

        if rmin < 0 { rmin = 0 };
        if cmin < 0 { cmin = 0 };
        if rmax > self.grid.len() as isize { rmax = self.grid.len() as isize };
        if cmax > self.grid[0].len() as isize { cmax = self.grid[0].len() as isize };

        for r in rmin..rmax {
            for c in cmin..cmax {
                let ur = r as usize;
                let uc = c as usize;
                if (ur, uc) != pos && self.grid[ur][uc] == look {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn resource_value(&self) -> usize {
        let mut wood = 0;
        let mut lumber = 0;

        for row in self.grid.iter() {
            for ch in row.iter() {
                match ch {
                    '|' => wood += 1,
                    '#' => lumber += 1,
                    _ => {}
                };
            }
        }

        lumber * wood
    }
}

impl fmt::Display for ConstructionArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.grid.iter() {
            let s : String = line.iter().collect();
            write!(f, "{}\n", s)?;
        };
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn day18_p1() {
        let mut c = ConstructionArea::load(INPUT);
        for i in 0..10 {
            c = c.next();
        }

        assert_eq!(c.resource_value(), 763804);
    }

    #[test]
    fn day18_p2() {
        let mut prev = ConstructionArea::load(INPUT);
        let mut cur : ConstructionArea;
        let mut seen : Vec<ConstructionArea> = vec![];


        for i in 1..1000000000 {
            cur = prev.next();
            seen.push(prev);

            if seen.contains(&cur) {
                let index = seen.iter().position(|v| *v == cur).unwrap();
                let period = (i - index);
                let remaining = (1000000000 - i) % (i - index);
                println!("found repeat after {} iterations with period {}", i, period);

                for _ in 0..(remaining) {
                    cur = cur.next();
                }
                assert_eq!(cur.resource_value(), 0);
                break;
            }

            prev = cur;
        }
    }
}
