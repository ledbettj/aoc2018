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

    pub fn tick(&mut self) -> Option<(isize, isize)> {
        self.carts.sort_by_key(|cart| cart.position);
        let mut crash = None;

        let new_carts = self.carts.iter().map(|c| {
            let (mut x, mut y) = c.position;
            let (mut dx, mut dy) = c.direction;
            let tmp = dx;
            let mut action : CartAction = c.action;

            x += dx;
            y += dy;

            if let Some(col) = self.carts.iter().find(|c| c.position == (x, y)) {
                if crash == None {
                    crash = Some((x, y));
                }
            }

            let ch = self.grid[y as usize][x as usize];
            match ch {
                '\\' => { dx = dy; dy = tmp; },
                '/'  => { dx = -dy; dy = -tmp; },
                // need to take action based on CartAction
                '+' => {
                    action = match c.action {
                        CartAction::Left => {
                            // swap ?
                            dx = dy; dy = -tmp;
                            CartAction::Straight
                        },
                        CartAction::Straight => {
                            // do nothing
                            CartAction::Right
                        },
                        CartAction::Right => {
                            // swap and invert
                            dx = -dy; dy = tmp;
                            CartAction::Left
                        }
                    }
                },
                '|' => { /* nop */ },
                '-' => { /* nop */ },
                x   => panic!(format!("wtf you doin on '{}'", x))
            };
            let mut new_cart = Cart::new(x, y, dx, dy);
            new_cart.action = action;
            new_cart
        }).collect();

        self.carts = new_carts;

        crash
    }
}


mod tests {
    use super::*;

    #[test]
    fn day13_p1() {
        let mut m = Minetracks::load(INPUT);
        loop {
            if let Some(pos) = m.tick() {
                assert_eq!(pos, (7,3));
                break;
            }
        }
    }

}
