

#[derive(Debug)]
struct RecipeBoard {
    scores: Vec<usize>,
    elves: Vec<usize>
}

impl RecipeBoard {
    pub fn new() -> RecipeBoard {
        RecipeBoard {
            scores: vec![3, 7],
            elves:  vec![0, 1]
        }
    }

    pub fn next(&mut self) {
        let s1 = self.scores[self.elves[0]];
        let s2 = self.scores[self.elves[1]];
        let sum = s1 + s2;
        let r1 = sum / 10;
        let r2 = sum % 10;

        if r1 != 0 {
            self.scores.push(r1);
        }

        self.scores.push(r2);

        self.elves[0] += s1 + 1;
        self.elves[1] += s2 + 1;

        self.elves[0] %= self.scores.len();
        self.elves[1] %= self.scores.len();
    }
}

mod tests {
    use super::*;

    #[test]
    fn day14_p1() {
        let mut r = RecipeBoard::new();
        let index = 607331;

        loop {
            r.next();

            if r.scores.len() > index + 10 {
                break;
            }
        }
        println!("{:?}", r.scores);

        let v = &r.scores[index..(index + 10)];
        assert_eq!(v, [8, 6, 1, 0, 3, 2, 1, 4, 1, 4]
    }

}
