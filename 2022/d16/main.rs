use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {}

#[derive(Debug, PartialEq)]
struct Input(HashMap<String, Valve>);

#[derive(Debug, PartialEq)]
struct Valve {
    id: String,
    flow_rate: i64,
    leads_to: Vec<String>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: HashMap<String, Valve> = HashMap::new();
        for l in s.lines() {
            let valve: Valve = l.parse()?;
            map.insert(valve.id.clone(), valve);
        }
        Ok(Input(map))
    }
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        let id = parts[1].to_string();
        let flow_rate_parts = parts[4];
        let flow_rate = flow_rate_parts
            .split("=")
            .nth(1)
            .map(|s| s.replace(";", ""))
            .map(|rate| rate.parse::<i64>().unwrap())
            .unwrap();
        let leads_to: Vec<String> = parts[9..]
            .into_iter()
            .map(|id| id.replace(",", ""))
            .collect();
        let input = Valve {
            id,
            flow_rate,
            leads_to,
        };
        Ok(input)
    }
}

impl Input {
    fn dfs(&self, id: String, seen: &mut HashSet<String>, current_flow: i64, minutes: i64) -> i64 {
        println!(
            "minute {}: @ {} with {} from {:?}",
            minutes, id, current_flow, seen
        );
        if minutes <= 0 {
            return current_flow;
        }
        if seen.contains(&id) {
            return current_flow;
        }
        if seen.len() == self.0.len() {
            return current_flow;
        }

        let v = self.0.get(&id).unwrap();

        // we moved here, which took a minute
        let minutes = minutes - 1;

        // either we can open
        let open_result = if v.flow_rate == 0 {
            Some((-1, HashSet::new()))
        } else {
            let mut seen = seen.clone();
            seen.insert(id.clone());

            let added_flow = minutes * v.flow_rate;
            let minutes = minutes - 1;
            let current_flow = current_flow + added_flow;

            let mut max: Option<(i64, HashSet<String>)> = None;
            let mut child_seen = seen.clone();
            for child in &v.leads_to {
                if child == &id {
                    continue;
                }
                let child_flow = self.dfs(child.clone(), &mut child_seen, current_flow, minutes);
                max = match max {
                    None => Some((child_flow, child_seen.clone())),
                    Some((current_max, current_seen)) => {
                        if child_flow > current_max {
                            Some((child_flow, child_seen.clone()))
                        } else {
                            Some((current_max, current_seen))
                        }
                    }
                }
            }
            max
        };

        // or not open
        let close_result = {
            let mut seen = seen.clone();
            // seen.insert(id.clone());

            let minutes = minutes - 0;
            let current_flow = current_flow + (minutes * 0);
            let mut max: Option<(i64, HashSet<String>)> = None;
            let mut child_seen = seen.clone();
            for child in &v.leads_to {
                if child == &id {
                    continue;
                }
                let child_flow = self.dfs(child.clone(), &mut child_seen, current_flow, minutes);
                max = match max {
                    None => Some((child_flow, child_seen.clone())),
                    Some((current_max, current_seen)) => {
                        if child_flow > current_max {
                            Some((child_flow, child_seen.clone()))
                        } else {
                            Some((current_max, current_seen))
                        }
                    }
                }
            }
            max
        };

        let max_close = close_result.map(|(i, _)| i).unwrap_or(-1);
        let max_open = open_result.map(|(i, _)| i).unwrap_or(-1);

        if max_close > max_open {
            println!("minute: {} flow: {}", minutes, max_close);
            max_close
        } else {
            println!("minute: {} flow: {}", minutes, max_open);
            max_open
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{Input, Valve};

    #[test]
    fn test_parse() {
        let input = include_str!("input.example.txt");
        let parsed: Input = input.parse().unwrap();

        assert_eq!(10, parsed.0.len());
    }

    #[test]
    fn test_path() {
        // AA
        // 	-> DD
        // 		-> CC
        // 			-> DD
        // 			-> BB
        // 				-> CC
        // 				-> AA
        // 		-> AA
        // 		-> EE
        // 			-> FF
        // 				-> EE
        // 				-> GG
        // 					-> FF
        // 					-> HH
        // 						-> GG
        // 			-> DD
        // 	-> II
        // 		-> AA
        // 		-> JJ
        // 			-> II
        // 	-> BB
        let input = include_str!("input.example.txt");
        let parsed: Input = input.parse().unwrap();
        println!("input: {:?}", parsed);
        // too slow :(
        // let result = parsed.dfs("AA".to_string(), &mut HashSet::new(), 0, 31);
        // println!("result: {}", result);
    }
}
