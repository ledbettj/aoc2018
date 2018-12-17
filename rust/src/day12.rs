use std::collections::HashMap;

const INPUT : &str = include_str!("../inputs/day12.txt");


struct PlantVec {
    plants: HashMap<isize, char>,
    rules: HashMap<String, char>
}

impl PlantVec {
    pub fn score(&self) -> isize {
        self.plants
            .iter()
            .filter(|&(&_, &v)| v == '#')
            .map(|(&k, &_)| k)
            .sum()
    }

    pub fn load(input: &str) -> PlantVec {
        let mut lines = input.lines();

        let mut rules : HashMap<String, char> = HashMap::new();
        let mut plants : HashMap<isize, char> = HashMap::new();

        let initial_state = lines
            .next()
            .unwrap()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .chars()
            .enumerate();

        for (index, chr) in initial_state {
            plants.insert(index as isize, chr);
        }

        for line in lines.skip(1) { // blank line
            let mut parts = line.split(" => ");
            let pattern = parts.next().unwrap().to_string();
            let result = parts.next().unwrap().chars().next().unwrap();
            rules.insert(pattern, result);
        }

        PlantVec {
            plants: plants,
            rules: rules
        }
    }

    pub fn next(&self) -> PlantVec {
        let mut next_plants : HashMap<isize, char> = HashMap::new();
        let mindex = self.plants.keys().min().unwrap() - 2;
        let maxdex = self.plants.keys().max().unwrap() + 2;

        for index in (mindex..maxdex) {
            let mut v : Vec<char> = Vec::new();

            v.push(*self.plants.get(&(index - 2)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index - 1)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index + 0)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index + 1)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index + 2)).unwrap_or(&'.'));

            let s : String = v.iter().collect();
            let rule = self.rules.get(&s).unwrap_or(&'.');
            next_plants.insert(index.clone(), *rule);
        }

        PlantVec {
            plants: next_plants,
            rules: self.rules.clone()
        }
    }

    pub fn inspect(&self) -> String {
        let mut k : Vec<isize> = self.plants.keys().map(|&k| k).collect();
        k.sort();

        let mut titles : String = k.iter()
            .map(|n| format!("{:02} ", n))
            .collect::<Vec<String>>()
            .join("");

        let mut values = k.iter()
            .map(|k| format!("{:02} ", *self.plants.get(k).unwrap()))
            .collect::<Vec<String>>()
            .join("");

        format!("{}\n{}", titles, values)
    }

}



mod test {
    use super::*;

    #[test]
    fn day12_p1() {
        let mut n = PlantVec::load(INPUT);

        for _ in (0..20) {
            n = n.next();
        }
        
        assert_eq!(n.score(), 1733);
    }
}
