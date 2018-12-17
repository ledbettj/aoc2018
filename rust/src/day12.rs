use std::collections::HashMap;

const INPUT : &str = include_str!("../inputs/day12.txt");


struct PlantVec {
    plants: HashMap<isize, char>,
    rules: HashMap<String, char>
}

impl PlantVec {
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

        for (index, state) in self.plants.iter() {
            let mut v : Vec<char> = Vec::new();
            
            v.push(*self.plants.get(&(index - 2)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index - 1)).unwrap_or(&'.'));
            v.push(*state);
            v.push(*self.plants.get(&(index + 1)).unwrap_or(&'.'));
            v.push(*self.plants.get(&(index + 2)).unwrap_or(&'.'));

            // let s = v.join("");

        }

        
        PlantVec {
            plants: next_plants,
            rules: self.rules.clone()
        }
    }

}



mod test {
    use super::*;

    #[test]
    fn day12_p1() {
        PlantVec::load(INPUT).next();
    }
}
