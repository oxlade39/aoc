use std::{collections::HashMap, str::FromStr, time::Instant};

use aoclib::input;
use aoclib::range::*;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("{}ms", now.elapsed().as_millis());
}

fn part1(txt: &str) -> usize {
    let sections: Vec<_> = input::empty_line_chunks(txt).collect();

    let workflows: HashMap<String, Workflow> = sections[0]
        .lines()
        .map(|l| l.parse::<Workflow>().unwrap())
        .map(|wf| (wf.name.clone(), wf))
        .collect();

    let mut sum = 0;
    let ratings = sections[1].lines().map(|l| l.parse::<Rating>().unwrap());
    let initial = "in".to_owned();

    for rating in ratings {
        let mut workflow = workflows.get(&initial).expect("in");
        while let Some(next) = workflow.rules.iter().find_map(|r| rating.apply(r)) {
            match next {
                Target::Workflow(wf) => {
                    workflow = workflows.get(&wf).expect(&format!("workflow {wf}"));
                }
                Target::Accept => {
                    sum += rating.total();
                    break;
                }
                Target::Reject => {
                    break;
                }
            }
        }
    }

    sum
}

fn part2(txt: &str) -> usize {
    let sections: Vec<_> = input::empty_line_chunks(txt).collect();

    let workflows: HashMap<String, Workflow> = sections[0]
        .lines()
        .map(|l| l.parse::<Workflow>().unwrap())
        .map(|wf| (wf.name.clone(), wf))
        .collect();

    let t: Target = Target::Workflow("in".to_owned());
    let mut state = State::new();
    let result = t.step(&mut state, &workflows);
    result
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Rule {
    LessThan(char, usize, Target),
    GreaterThan(char, usize, Target),
    Target(Target),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    x: InclusionRange,
    m: InclusionRange,
    a: InclusionRange,
    s: InclusionRange,
}

impl State {
    fn new() -> Self {
        let r = InclusionRange::new(1, 4001);
        Self {
            x: r.clone(),
            m: r.clone(),
            a: r.clone(),
            s: r.clone(),
        }
    }

    fn length(&self) -> usize {
        self.x.length() * self.m.length() * self.a.length() * self.s.length()
    }

    fn update_less_than(&mut self, c: char, less_than: usize) {
        match c {
            'x' => {
                self.x.update_less_than(less_than);
            }
            'm' => {
                self.m.update_less_than(less_than);
            }
            'a' => {
                self.a.update_less_than(less_than);
            }
            's' => {
                self.s.update_less_than(less_than);
            }
            _ => panic!("bad character"),
        }
    }

    fn update_more_than(&mut self, c: char, less_than: usize) {
        match c {
            'x' => {
                self.x.update_more_than(less_than);
            }
            'm' => {
                self.m.update_more_than(less_than);
            }
            'a' => {
                self.a.update_more_than(less_than);
            }
            's' => {
                self.s.update_more_than(less_than);
            }
            _ => panic!("bad character"),
        }
    }
}

fn step(state: &mut State, workflow: &Workflow, workflows: &HashMap<String, Workflow>) -> usize {
    let mut total = 0;
    for r in &workflow.rules {
        let contributes = match r {
            Rule::LessThan(c, n, t) => {
                let mut left = state.clone();

                left.update_less_than(*c, *n);
                state.update_more_than(*c, *n - 1);

                t.step(&mut left, workflows)
            }
            Rule::GreaterThan(c, n, t) => {
                let mut left = state.clone();

                left.update_more_than(*c, *n);
                state.update_less_than(*c, *n + 1);

                t.step(&mut left, workflows)
            }
            Rule::Target(t) => t.step(state, workflows),
        };
        total += contributes;
    }
    total
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Target {
    Workflow(String),
    Accept,
    Reject,
}

impl Target {
    fn step(&self, state: &mut State, workflows: &HashMap<String, Workflow>) -> usize {
        match self {
            Target::Workflow(wf) => {
                let next = workflows.get(wf).unwrap();
                step(state, next, workflows)
            }
            Target::Accept => state.length(),
            Target::Reject => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn total(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn apply(&self, rule: &Rule) -> Option<Target> {
        match rule {
            Rule::LessThan('x', n, t) if self.x < *n => Some(t.clone()),
            Rule::LessThan('m', n, t) if self.m < *n => Some(t.clone()),
            Rule::LessThan('a', n, t) if self.a < *n => Some(t.clone()),
            Rule::LessThan('s', n, t) if self.s < *n => Some(t.clone()),

            Rule::GreaterThan('x', n, t) if self.x > *n => Some(t.clone()),
            Rule::GreaterThan('m', n, t) if self.m > *n => Some(t.clone()),
            Rule::GreaterThan('a', n, t) if self.a > *n => Some(t.clone()),
            Rule::GreaterThan('s', n, t) if self.s > *n => Some(t.clone()),

            Rule::Target(t) => Some(t.clone()),

            _ => None,
        }
    }
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut outer = s.split("{");
        let name = outer.next().unwrap().to_owned();
        let rest = outer.next().unwrap().replace("}", "");
        let rules: Vec<_> = rest
            .split(",")
            .map(|rule| rule.parse::<Rule>().unwrap())
            .collect();

        Ok(Workflow { name, rules })
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn get_target(s: &str) -> Target {
            match s {
                "A" => Target::Accept,
                "R" => Target::Reject,
                other => Target::Workflow(other.to_owned()),
            }
        }

        let parts: Vec<_> = s.split(":").collect();
        if parts.len() == 1 {
            Ok(Rule::Target(get_target(parts[0])))
        } else {
            let left = parts[0];

            match left.split_once("<") {
                Some((var, i)) => Ok(Rule::LessThan(
                    var.chars().next().unwrap(),
                    i.parse().unwrap(),
                    get_target(parts[1]),
                )),
                None => Err("not less than".to_owned()),
            }
            .or(match left.split_once(">") {
                Some((var, i)) => Ok(Rule::GreaterThan(
                    var.chars().next().unwrap(),
                    i.parse().unwrap(),
                    get_target(parts[1]),
                )),
                None => Err("not less than or greater than".to_owned()),
            })
        }
    }
}

impl FromStr for Rating {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.replace("{", "").replace("}", "");
        let each_rating: Vec<_> = cleaned.split(",").collect();

        let x = each_rating[0]
            .split("=")
            .nth(1)
            .expect("=")
            .parse::<usize>()
            .expect("x value");
        let m = each_rating[1]
            .split("=")
            .nth(1)
            .expect("=")
            .parse::<usize>()
            .expect("x value");
        let a = each_rating[2]
            .split("=")
            .nth(1)
            .expect("=")
            .parse::<usize>()
            .expect("x value");
        let s = each_rating[3]
            .split("=")
            .nth(1)
            .expect("=")
            .parse::<usize>()
            .expect("x value");

        Ok(Self { x, m, a, s })
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_example_p1() {
        assert_eq!(19114, part1(include_str!("input.test.txt")));
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(167409079868000, part2(include_str!("input.test.txt")));
    }

    #[test]
    fn test_parse_rule_less_than_workflow() {
        let str = "a<2006:qkq";
        let rule = str.parse::<Rule>();
        assert_eq!(
            Ok(Rule::LessThan(
                'a',
                2006,
                Target::Workflow("qkq".to_owned())
            )),
            rule
        );
    }

    #[test]
    fn test_parse_rule_greater_than_reject() {
        let str = "a>1716:R";
        let rule = str.parse::<Rule>();
        assert_eq!(Ok(Rule::GreaterThan('a', 1716, Target::Reject)), rule);
    }

    #[test]
    fn test_parse_rule_greater_than_accept() {
        let str = "m>1548:A";
        let rule = str.parse::<Rule>();
        assert_eq!(Ok(Rule::GreaterThan('m', 1548, Target::Accept)), rule);
    }

    #[test]
    fn test_parse_workflow() {
        let s = "px{a<2006:qkq,m>2090:A,rfg}";
        let parsed = s.parse::<Workflow>();
        let expected = Ok(Workflow {
            name: "px".to_owned(),
            rules: vec![
                Rule::LessThan('a', 2006, Target::Workflow("qkq".to_owned())),
                Rule::GreaterThan('m', 2090, Target::Accept),
                Rule::Target(Target::Workflow("rfg".to_owned())),
            ],
        });

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_parse_workflow_end_state() {
        let s = "qqz{s>2770:qs,m<1801:hdj,R}";
        let parsed = s.parse::<Workflow>();
        let expected = Ok(Workflow {
            name: "qqz".to_owned(),
            rules: vec![
                Rule::GreaterThan('s', 2770, Target::Workflow("qs".to_owned())),
                Rule::LessThan('m', 1801, Target::Workflow("hdj".to_owned())),
                Rule::Target(Target::Reject),
            ],
        });

        assert_eq!(expected, parsed);
    }

    #[test]
    fn test_parse_rating() {
        let s = "{x=787,m=2655,a=1222,s=2876}";
        let parsed = s.parse::<Rating>();

        assert_eq!(
            Ok(Rating {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            }),
            parsed
        )
    }

    #[test]
    fn test_initial_state_length() {
        let s = State::new();
        assert_eq!(4000 * 4000 * 4000 * 4000, s.length());
    }

    #[test]
    fn test_start_point() {
        let wf_in: Workflow = "in{A}".parse().unwrap();
        let workflows: HashMap<_, _> = vec![("in".to_owned(), wf_in)].into_iter().collect();
        let result = Target::Workflow("in".to_owned()).step(&mut State::new(), &workflows);

        assert_eq!(4000 * 4000 * 4000 * 4000, result);
    }

    #[test]
    fn test_updating_state_simple() {
        let wf_in: Workflow = "in{s<21:A,R}".parse().unwrap();
        let workflows: HashMap<_, _> = vec![("in".to_owned(), wf_in)].into_iter().collect();
        let result = Target::Workflow("in".to_owned()).step(&mut State::new(), &workflows);

        assert_eq!(20 * 4000 * 4000 * 4000, result);
    }

    #[test]
    fn test_updating_state_simple_reject() {
        let wf_in: Workflow = "in{s>20:R,A}".parse().unwrap();
        let workflows: HashMap<_, _> = vec![("in".to_owned(), wf_in)].into_iter().collect();
        let result = Target::Workflow("in".to_owned()).step(&mut State::new(), &workflows);

        assert!(result < (4000 * 4000 * 4000 * 4000));
        assert!(result > 0);
        assert_eq!(20 * 4000 * 4000 * 4000, result);
    }
}
