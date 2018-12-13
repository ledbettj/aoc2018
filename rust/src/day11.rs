const DAY11_INPUT : isize =  5093;


fn score_at(x: isize, y: isize) -> isize {
    let rack_id : isize = (x + 10) as isize;
    let mut power : isize = rack_id * (y as isize);
    power += DAY11_INPUT;
    power *= rack_id;
    let hundred = (power % 1000) / 100;
    (hundred - 5)
}

fn score_at_3x3(x: isize, y: isize) -> isize {
    let mut sum = 0;

    for dx in (x..(x+3)) {
        for dy in (y..(y+3)) {
            sum += score_at(dx, dy);
        }
    }

    sum
}

fn find_most_powerful_3x3() -> (isize, isize) {
    let mut best : (isize, isize) = (0, 0);
    let mut score = score_at_3x3(0, 0);

    for x in (0..297) {
        for y in (0..297) {
            let new_score = score_at_3x3(x, y);
            if new_score > score {
                score = new_score;
                best = (x, y);
            }
        }
    }
    best
}

mod tests {
    use super::*;

    #[test]
    fn day11_p1() {
        assert_eq!(find_most_powerful_3x3(), (243, 49));
    }

}
