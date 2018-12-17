use regex::Regex;

const INPUT : &str = include_str!("../inputs/day10.txt");


#[derive(Debug,Copy,Clone)]
struct Star {
    pub position: (isize, isize),
    pub velocity: (isize, isize)
}


impl Star {
    pub fn new(pos: (isize, isize), vel: (isize, isize)) -> Star {
        Star { position: pos, velocity: vel }
    }

    pub fn tick(&mut self) {
        let (ref mut x, ref mut y) = &mut self.position;
        let &(vx, vy) = &self.velocity;
        *x += vx;
        *y += vy;
    }

    pub fn distance(&self, other: &Star) -> isize {
        (self.position.0 - other.position.0).abs() + (self.position.1 - other.position.1).abs()
    }

    pub fn load(input: &str) -> Vec<Star> {
        let r = Regex::new(r"position=<\s*([^,]+),\s*([^>]+)>\s*velocity=<\s*([^,]+),\s*([^>]+)>").unwrap();
        input
            .lines()
            .map(|line| {
                let caps : Vec<isize> = r.captures(line)
                    .expect(&format!("Failed to match regex w '{}'", line))
                    .iter()
                    .skip(1)
                    .map(|m| {
                        let s = m.unwrap().as_str();
                        s.parse::<isize>().expect(&format!("Failed to parse number from '{}'", s))
                    })
                    .collect();

                Star::new((caps[0], caps[1]), (caps[2], caps[3]))
            })
            .collect()
    }

    pub fn is_interesting_field(stars: &Vec<Star>) -> bool {
        let s = stars[0];

        let close = stars.iter().skip(1).filter(|star| s.distance(star) < 5).count();
        close > 10
    }

    pub fn tick_all(stars: &mut Vec<Star>) {
        for mut star in stars.iter_mut() {
            star.tick();
        };
    }

    pub fn dump(stars: &mut Vec<Star>) {
        let mut minx = stars.iter().min_by_key(|a| a.position.0).unwrap().position.0;
        let mut miny = stars.iter().min_by_key(|a| a.position.1).unwrap().position.1;

        if minx < 0 {
            minx = -minx;
        }

        if miny < 0 {
            miny = -miny;
        }

        for mut star in stars.iter_mut() {
            let (ref mut x, ref mut y) = &mut star.position;
            *x -= minx;
            *y -= miny;
        }

        let maxx = stars.iter().max_by_key(|a| a.position.0).unwrap().position.0 + 2;
        let maxy = stars.iter().max_by_key(|a| a.position.1).unwrap().position.1 + 2;

        for y in (0..maxy) {
            for x in (0..maxx) {
                if stars.iter().filter(|s| s.position == (x, y)).count() > 0 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

    }
}


mod tests {
    use super::*;

    #[test]
    fn day10_p1() {
        let mut starfield = Star::load(INPUT);

        loop {
            Star::tick_all(&mut starfield);

            if Star::is_interesting_field(&starfield) {
                Star::dump(&mut starfield);
                std::thread::sleep_ms(1_000);
            }
        }

        
    }
}
