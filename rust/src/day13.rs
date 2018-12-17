const INPUT : &str = include_str!("../inputs/day13.txt");
const SAMPLE : &str = include_str!("../inputs/day13-sample.txt");

#[derive(Debug,Clone,Copy)]
enum CartAction {
    Left,
    Straight,
    Right
}

#[derive(Debug,Clone,Copy)]
struct Cart {
    position: (isize, isize),
    direction: (isize, isize),
    action: CartAction
}

#[derive(Debug)]
struct Minetracks  {
    grid: Vec<Vec<char>>,
    carts: Vec<Cart>
}

impl Cart {
    pub fn new(x: isize, y: isize, dx: isize, dy: isize) -> Cart {
        Cart {
            position: (x, y),
            direction: (dx, dy),
            action: CartAction::Left
        }
    }
}


impl Minetracks {
    pub fn load(input: &str) -> Minetracks {
        let mut carts : Vec<Cart> = vec![];
        let grid = input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                let chars : Vec<char> = line
                    .chars()
                    .enumerate()
                    .map(|(col, ch)| {
                        let x = col as isize;
                        let y = row as isize;
                        match ch {
                            '^' => { carts.push(Cart::new(x, y, 0, -1)); '|' },
                            'v' => { carts.push(Cart::new(x, y, 0,  1)); '|' },
                            '<' => { carts.push(Cart::new(x, y, -1, 0)); '-' },
                            '>' => { carts.push(Cart::new(x, y, 1,  0)); '-' },
                            x => x
                        }
                    })
                    .collect();

                chars
            })
            .collect();

        Minetracks { grid: grid, carts: carts }
    }

    pub fn crash(&self) -> Option<(isize, isize)> {
        for i in 0..self.carts.len() {
            for j in (i+1)..self.carts.len() {
                if self.carts[i].position == self.carts[j].position {
                    return Some(self.carts[i].position);
                }
            }
        };
        None
    }

    pub fn tick(&mut self) -> Option<(isize, isize)> {
        self.carts.sort_by_key(|cart| (cart.position.1, cart.position.0));
        let mut seen : Vec<(isize, isize)> = vec![];

        for c in self.carts.iter_mut() {
            let (ref mut x, ref mut y) = &mut c.position;
            let (ref mut dx, ref mut dy) = &mut c.direction;
            let tmp = *dx;

            // move cart
            *x += *dx;
            *y += *dy;
            let ch = self.grid[*y as usize][*x as usize];
            match ch {
                '\\' => { *dx = *dy; *dy = tmp; },
                '/'  => { *dx = -*dy; *dy = -tmp; },
                // need to take action based on CartAction
                '+' => {
                    c.action = match c.action {
                        CartAction::Left => {
                            // swap ?
                            *dx = *dy; *dy = tmp;
                            CartAction::Straight
                        },
                        CartAction::Straight => {
                            // do nothing
                            CartAction::Right
                        },
                        CartAction::Right => {
                            // swap and invert
                            *dx = -*dy; *dy = -tmp;
                            CartAction::Left
                        }
                    }
                },
                '|' => { /* nop */ },
                '-' => { /* nop */ },
                x   => panic!(format!("wtf you doin on '{}'", x))
            };
            if seen.contains(&(*x, *y)) {
                return Some((*x, *y));
            }
            seen.push((*x, *y));
        };

        None
    }
}


mod tests {
    use super::*;

    #[test]
    fn day13_p1() {
        let mut m = Minetracks::load(INPUT);
        loop {
            if let Some(pos) = m.tick() {
                assert_eq!(pos, (7, 3));
                break;
            }
        }
    }

}
