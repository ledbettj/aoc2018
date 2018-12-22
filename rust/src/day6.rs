
use std::collections::HashMap;

const DAY6_INPUT : &str = include_str!("../inputs/day6.txt");


fn manhattan(p1: (isize, isize), p2: (isize, isize)) -> isize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    (x1 - x2).abs() + (y1 - y2).abs()
}

struct Grid {
    points: Vec<(isize, isize)>,
    grid:   Vec<Vec<isize>>
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let mut list = Grid::parse_points_list(input);
        // adjust the list to start at 1,1
        let xmin = list.iter().min_by_key(|point| point.0).unwrap().0;
        let ymin = list.iter().min_by_key(|point| point.1).unwrap().1;

        list = list
            .iter()
            .map(|(x, y)| (x - (xmin - 1), y - (ymin - 1)))
            .collect();

        // build the grid from 0,0 to (xmax, ymax)
        let xmax = list.iter().max_by_key(|point| point.0).unwrap().0;
        let ymax = list.iter().max_by_key(|point| point.1).unwrap().1;

        let mut grid : Vec<Vec<isize>> = (0..xmax).map(|_| vec![0; ymax as usize] ).collect();

        Grid {
            points: list,
            grid: grid
        }
    }

    pub fn safest_area_size(&mut self) -> usize {
        let mut counts = HashMap::new();
        let mut total = 0;

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let counter = counts.entry((x,y)).or_insert(0);
                *counter = self.points
                    .iter()
                    .fold(0, |accum, &p| accum + manhattan(p, (x as isize, y as isize)));

                if *counter < 10_000 {
                    total += 1;
                }
            }
        }
        total
    }

    pub fn largest_noninfinite_area_size(&self) -> usize {
        // index = index of point,
        // value = number of points in grid for that point
        let mut counts = HashMap::new();

        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                let v = self.grid[x][y];
                let counter = counts.entry(v).or_insert(0);
                *counter += 1;
            }
        };

        for x in 0..self.grid.len() {
            counts.remove(&self.grid[x][0]);
            counts.remove(&self.grid[x][self.grid[x].len() - 1]);
        }

        for y in 0..self.grid[0].len() {
            counts.remove(&self.grid[0][y]);
            counts.remove(&self.grid[self.grid.len() - 1][y]);
        }

        counts.iter().max_by_key(|&(k, v)| v).unwrap().1.clone()
    }


    fn fill_distance_grid(&mut self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {

                let mut distances : Vec<(usize, isize)> = self.points
                    .iter()
                    .enumerate()
                    .map(|(index, p)| (index, manhattan(*p, (x as isize, y as isize))))
                    .collect();

                distances.sort_by_key(|&(_index, distance)| distance);
                if distances[1].1 == distances[0].1 {
                    // at least 2 points same distance away
                    self.grid[x][y] = -1
                } else {
                    self.grid[x][y] = distances[0].0 as isize;
                }
            }
        }
    }

    fn print(&self) {
        for x in 0..self.grid.len() {
            for y in 0..self.grid[x].len() {
                print!("{:02} ", self.grid[x][y]);
            }
            println!("");
        }
    }

    fn parse_points_list(input: &str) -> Vec<(isize, isize)> {
        input
            .lines()
            .map(|line|{
                let parts : Vec<isize> = line
                    .split(", ")
                    .map(|part| part.parse::<isize>().unwrap())
                    .collect();

                (parts[0], parts[1])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_p1_test() {
        let mut g = Grid::new(DAY6_INPUT);
        g.fill_distance_grid();
        assert_eq!(g.largest_noninfinite_area_size(), 3010);
    }

    #[test]
    fn day6_p2() {
        let mut g = Grid::new(DAY6_INPUT);
        g.fill_distance_grid();

        assert_eq!(g.safest_area_size(), 0);

    }
}
