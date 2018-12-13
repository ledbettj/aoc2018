use std::collections::HashMap;
use regex::Regex;

const DAY7_INPUT : &str = include_str!("../inputs/day7.txt");

#[derive(Debug)]
struct SleighPlans {
    dependencies: HashMap<String, Vec<String>>,
    in_progress:  HashMap<String, usize>,
    workers:      usize
}


impl SleighPlans {
    pub fn load(input: &str) -> SleighPlans {
        SleighPlans {
            dependencies: SleighPlans::load_dependencies(input),
            in_progress: HashMap::new(),
            workers: 5
        }
    }


    fn load_dependencies(input: &str) -> HashMap<String, Vec<String>> {
        let mut result : HashMap<String, Vec<String>> = HashMap::new();
        let r = Regex::new(r"Step (?P<step>\w) must be finished before step (?P<dep>\w) can begin\.").unwrap();
        for line in input.lines() {
            let caps = r.captures(line).unwrap();
            let step = (&caps["step"]).to_owned();
            let dep  = (&caps["dep"]).to_owned();
            let entry = result.entry(step).or_insert(Vec::new());

            entry.push(dep.clone());

            result.entry(dep).or_insert(Vec::new());
        }

        result
    }

    fn available_steps(&self) -> Vec<String> {
        let mut available : Vec<String> = self.dependencies
            .iter()
            .filter(|(key, val)| val.is_empty() && !self.in_progress.contains_key(*key))
            .map(|(key, val)| key.clone())
            .collect();

        available.sort();
        available
    }

    fn cost_for(c: char) -> usize {
        61 + (c as usize) - ('A' as usize)
    }


    fn assemble(&mut self) -> usize {
        let mut tick : usize = 0;

        while !self.dependencies.is_empty() {
            let available_steps = self.available_steps();
            let free_workers    = self.workers - self.in_progress.len();

            for avail in available_steps.iter().take(free_workers) {
                self.in_progress.insert(avail.to_string(),
                                        SleighPlans::cost_for(avail.chars().nth(0).unwrap()));
            }

            tick += 1;

            // decrement work time
            for (k, v) in self.in_progress.iter_mut() {
                *v -= 1;
                if *v == 0 {
                    self.dependencies.remove(k);
                    for (dk, dkv) in self.dependencies.iter_mut() {
                        dkv.retain(|ch| *ch != *k);
                    }
                }
            }

            // remove done jobs from in_progress
            self.in_progress.retain(|k, v| *v > 0);
        }

        tick
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day72_p2_test() {
        let mut plans = SleighPlans::load(DAY7_INPUT);
        assert_eq!(plans.assemble(), 0);
    }
}
