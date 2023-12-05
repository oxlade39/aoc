use std::{str::FromStr, collections::HashMap, i64};

use itertools::Itertools;


fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

fn part1(txt: &str) -> i64 {
    // TODO make os portable (carriage returns)
    let sep = "\r\n\r\n";

    let mut parts = txt.split(sep);

    let seeds = parts.next()
        .expect("first row")
        .parse::<Seeds>().expect("seeds");

    let mut graph: HashMap<String, Mapping> = HashMap::new();

    for m in parts
        .map(|p| p.parse::<Mapping>().expect("mapping")) {
            let clone = m.clone();
            graph.insert(
                clone.from.to_owned(), 
                clone
            );
    }
        
    let locations = seeds.0.iter()
        .map(|s| {
            let mut key = "seed";
            let mut n = *s;
            while let Some(next) = graph.get(key){
                key = &next.to;
                n = next.resolve(n);
            }
            n
        })
        .collect_vec();

    *locations.iter().min().unwrap()
} 

fn part2(txt: &str) -> i64 {
    let sep = "\r\n\r\n";

    let mut parts = txt.split(sep);

    let seeds = parts.next()
        .expect("first row")
        .parse::<SeedRanges>().expect("seeds");

    let mut graph: HashMap<String, Mapping> = HashMap::new();

    for m in parts
        .map(|p| p.parse::<Mapping>().expect("mapping")) {
            let clone = m.clone();
            graph.insert(
                clone.to.to_owned(), 
                clone
            );
    }

    // let locations = 46..;
    let mut location = 0;
    let increment = if seeds.0[0].0 > 1000 {
        1000
    } else {
        10
    };

    loop {
        match find(location, &graph, &seeds) {
            Some(_) => {
                break;
            },
            None => {
                location += increment;
            }
        }
    }
    
    loop {
        match find(location, &graph, &seeds) {
            Some(_) => {
                location -= 1;
            },
            None => {
                break;
            }
        }
    }
    location + 1    
}

fn find(
    location: i64, 
    graph: &HashMap<String, Mapping>,
    seeds: &SeedRanges,
) -> Option<i64> {
    let mut key = "location";
    let mut n = location;
    while let Some(next) = graph.get(key) {
        key = &next.from;
        n = next.reverse_resolve(n);
    }
    if seeds.within(n) {
        Some(location)
    } else {
        None
    }
}

#[derive(Debug)]
struct Seeds(Vec<i64>);

impl FromStr for Seeds {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outter = s.split(": ").collect_vec();
        let inner: Vec<i64> = outter[1].split(" ")
            .map(|n| n.parse::<i64>().expect("seed number"))
            .collect();
        Ok(Seeds(inner))
    }
}

#[derive(Debug)]
struct SeedRanges(Vec<(i64, i64)>);

impl SeedRanges {
    fn within(&self, i: i64) -> bool {
        self.0.iter()
            .any(|(n, length)| {
                let lower = *n;
                let upper = *n + length;
                i >= lower && i < upper
            })
    }
}

impl FromStr for SeedRanges {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let outter = s.split(": ").collect_vec();
        let inner = outter[1].split(" ")
            .map(|n| n.parse::<i64>().expect("seed number"))
            .chunks(2)
            .into_iter()
            .map(|c| {
                let parts = c.collect_vec();
                (parts[0], parts[1])
            })
            .collect_vec();
        
        Ok(SeedRanges(inner))
    }
}

#[derive(Debug, Clone)]
struct Range {
    source: i64,
    destination: i64,
    length: i64,
}

impl Range {
    fn resolve(&self, source: i64) -> Option<i64> {
        let delta = source - self.source;
        if delta >= 0 && delta < self.length {
            return Some(self.destination + delta);
        }
        None
    }

    fn reverse_resolve(&self, target: i64) -> Option<i64> {
        let delta = target - self.destination;
        if delta >= 0 && delta < self.length {
            return Some(self.source + delta);
        }
        None
    }
}

impl FromStr for Range {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect_vec();
        if parts.len() != 3 {
            panic!("bad range: {}", s);
        }
        Ok(Range {
            source: parts[1].parse::<i64>().expect(&format!("{}", parts[1])),
            destination: parts[0].parse::<i64>().unwrap(),
            length: parts[2].parse::<i64>().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Mapping {
    fn resolve(&self, source: i64) -> i64 {
        match self.ranges.iter()
            .filter_map(|range| range.resolve(source))
            .next() {
                Some(x) => x,
                _ => source
            }
    }

    fn reverse_resolve(&self, target: i64) -> i64 {
        match self.ranges.iter()
            .filter_map(|range| range.reverse_resolve(target))
            .next() {
                Some(x) => x,
                _ => target
            }
    }
}

impl FromStr for Mapping {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heading = s.lines().next().unwrap();
        let name_part =heading.split(" ").next().unwrap();
        let key_part = name_part.split("-").collect_vec();
        let from = key_part[0].to_owned();
        let to = key_part[2].to_owned();

        let ranges = s.lines()
            .skip(1)
            .map(|l| l.parse::<Range>().expect("range"))
            .collect_vec();

        Ok(Mapping { from, to, ranges })
    }
}


#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    fn test_example_p1() {
        assert_eq!(35, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(46, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_range() {
        let r: Range = "50 98 2".parse().unwrap();

        assert_eq!(50, r.destination);
        assert_eq!(98, r.source);
        assert_eq!(2, r.length);

        assert_eq!(Some(50), r.resolve(98));
        assert_eq!(Some(51), r.resolve(99));
        assert_eq!(None, r.resolve(100));
    }

    #[test]
    fn test_range_reverse() {
        let r: Range = "50 98 2".parse().unwrap();

        assert_eq!(50, r.destination);
        assert_eq!(98, r.source);
        assert_eq!(2, r.length);

        assert_eq!(Some(98), r.reverse_resolve(50));
        assert_eq!(Some(99), r.reverse_resolve(51));
        assert_eq!(None, r.reverse_resolve(52));
    }

    #[test]
    fn test_mapping() {
        let soil: Mapping = "seed-to-soil map:\n\
        50 98 2\n\
        52 50 48".parse().unwrap();

        let fertilizer: Mapping = "soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15".parse().unwrap();

        assert_eq!(81, soil.resolve(79));
        assert_eq!(81, fertilizer.resolve(81));
    }

    #[test]
    fn test_mapping_reverse() {
        let soil: Mapping = "seed-to-soil map:\n\
        50 98 2\n\
        52 50 48".parse().unwrap();

        let fertilizer: Mapping = "soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15".parse().unwrap();

        assert_eq!(79, soil.reverse_resolve(81));
        assert_eq!(81, fertilizer.reverse_resolve(81));
    }

    #[test]
    fn test_full_example_pt2() {
        let input = include_str!("input.test.txt");
        let sep = "\r\n\r\n";
        let mappings = input.split(sep).skip(1)
            .map(|chunk| chunk.parse::<Mapping>().unwrap())
            .collect_vec();

        // In the above example, the lowest location number can be 
        // obtained from seed number 
        // 82, which corresponds to soil 
        // 84, fertilizer 
        // 84, water 
        // 84, light 
        // 77, temperature 
        // 45, humidity 
        // 46, and location 46. 
        // So, the lowest location number is 46

        let expects: [i64; 7] = [
            82,
            84,
            84,
            84,
            77,
            45,
            46
        ];

        let r: Range = "45 77 23".parse().unwrap();
        assert_eq!(Some(45), r.resolve(77));

        println!("***************");
        for m in &mappings {
            println!("{:?}", m);
        }
        println!("***************");
        

        assert_eq!("light", mappings[4].from);
        assert_eq!("temperature", mappings[4].to);
        assert_eq!(45, mappings[4].resolve(77));

        let mut i = 82;
        for m in mappings {
            let next = m.resolve(i);
            println!("from {} {} to {} {}", m.from, i, m.to, next);
            i = next;
        }
    }

    #[test]
    fn test_regression() {
        assert_eq!(309796150, part1(include_str!("input.txt")));
        assert_eq!(50716416, part2(include_str!("input.txt")));
    }
}
