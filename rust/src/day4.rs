
use regex::Regex;

const INPUT : &str = include_str!("../inputs/day4.txt");

enum Event {
    NewGuard { id: u32, hour: u32, min: u32 },
    Asleep { id: u32, hour: u32, min: u32 },
    Awake { id: u32, hour: u32, min: u32 }
}

// fn load_events(input: &str) -> Vec<Event> {
//     let re = Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] (\w+) #?(\w+)").unwrap();
//     input
//         .lines()
//         .map(|line|{
            


//         })
//         .collect();

// }


mod tests {
    use super::*;

    #[test]
    fn day4_p1() {
        

    }

}
