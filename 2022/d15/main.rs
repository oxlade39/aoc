use std::{collections::HashSet, fmt::Debug, str::FromStr, time::Instant};

use aoclib::{
    cartesian::{Plane, Point, Transform},
    distance::{Distance, ManhattenDistance},
};

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let part1 = part1(input, 2000000);
    println!("part1: {:?}", part1);
    let start = Instant::now();
    let part2 = part2(input, (4000000, 4000000).into());
    let duration = start.elapsed();
    println!("part2: {:?} - took {:?}", part2, duration);
}

struct Sensor {
    pos: Point,
    closest_beacon: Point,
}

struct Map {
    sensors: Vec<Sensor>,
    sensor_points: HashSet<Point>,
    beacons: HashSet<Point>,
    plane: Plane,
}

impl Sensor {
    fn beacon_distance(&self) -> ManhattenDistance {
        ManhattenDistance::from_vector((self.pos.clone(), self.closest_beacon.clone()).into())
    }

    fn y_border_points(&self, y: i64) -> Option<(Point, Point)> {
        let ManhattenDistance(max_axes_delta) = self.beacon_distance();
        let distance_to_y = y - self.pos.y;
        let abs_distance_to_y = distance_to_y.abs();

        if abs_distance_to_y > max_axes_delta {
            None
        } else {
            let x_delta = (max_axes_delta - abs_distance_to_y).abs();
            let t: [Transform; 2] = [
                (-x_delta, distance_to_y).into(),
                (x_delta, distance_to_y).into(),
            ];
            Some((self.pos.transform(&t[0]), self.pos.transform(&t[1])))
        }
    }
}

impl Map {
    fn exclusion_area(&self, y: i64, min_x: i64, max_x: i64) -> ExclusionArea {
        let mut exlusion_cols = ExclusionArea::default();
        for sensor in &self.sensors {
            if let Some((left, right)) = sensor.y_border_points(y) {
                let left_x = left.x.max(min_x);
                let right_x = right.x.min(max_x);
                exlusion_cols.exclude((left_x, right_x));
            }
        }
        exlusion_cols
    }
}

fn part1(input: &str, y: i64) -> i64 {
    let map: Map = input.parse().unwrap();
    map.exclusion_area(y, -y * 5, y * 5).exclusion_size - 1
}

fn part2(input: &str, max: Point) -> i64 {
    let map: Map = input.parse().unwrap();
    let result = (0..=max.y).into_par_iter().find_map_any(|y| {
        let exlusion_cols = map.exclusion_area(y, 0, max.x);
        if exlusion_cols.exclusion_size <= max.x {
            let mut to_sort = exlusion_cols.exclusions.clone();
            to_sort.sort_by(|(_, left), (_, right)| (*left).cmp(right));
            let x = to_sort[0].1 + 1;
            Some(x * 4000000 + y)
        } else {
            None
        }
    });
    result.unwrap_or(-1)
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sensors = vec![];
        let mut sensor_points: HashSet<Point> = HashSet::new();
        let mut beacons: HashSet<Point> = HashSet::new();
        let mut mins = [0; 2];
        let mut maxes = [0; 2];
        for line in s.lines() {
            let parts: Vec<_> = line.split(" ").collect();
            let sensor_x: i64 = parts[2]
                .split("=")
                .nth(1)
                .unwrap()
                .replace(",", "")
                .parse()
                .unwrap();
            let sensor_y: i64 = parts[3]
                .split("=")
                .nth(1)
                .unwrap()
                .replace(":", "")
                .parse()
                .unwrap();
            let beacon_x: i64 = parts[8]
                .split("=")
                .nth(1)
                .unwrap()
                .replace(",", "")
                .parse()
                .unwrap();
            let beacon_y: i64 = parts[9].split("=").nth(1).unwrap().parse().unwrap();

            let sensor = (sensor_x, sensor_y).into();
            let beacon = (beacon_x, beacon_y).into();
            sensors.push(Sensor {
                pos: (sensor_x, sensor_y).into(),
                closest_beacon: (beacon_x, beacon_y).into(),
            });
            mins[0] = mins[0].min(sensor_x).min(beacon_x);
            mins[1] = mins[1].min(sensor_y).min(beacon_y);
            maxes[0] = maxes[0].max(sensor_x).max(beacon_x);
            maxes[1] = maxes[1].max(sensor_y).max(beacon_y);
            sensor_points.insert(sensor);
            beacons.insert(beacon);
        }

        Ok(Map {
            sensors,
            sensor_points,
            beacons,
            plane: Plane {
                top_left: (mins[0], maxes[1]).into(),
                bottom_right: (maxes[0], mins[1]).into(),
            },
        })
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let p = &self.plane;
        for y in p.bottom_right.y..=p.top_left.y {
            for x in p.top_left.x..=p.bottom_right.x {
                let point: Point = (x, y).into();
                if self.beacons.contains(&point) {
                    write!(f, "B")?;
                } else if self.sensor_points.contains(&point) {
                    write!(f, "S")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct ExclusionArea {
    exclusion_size: i64,
    exclusions: Vec<(i64, i64)>,
}

impl Default for ExclusionArea {
    fn default() -> Self {
        Self {
            exclusion_size: 0,
            exclusions: vec![],
        }
    }
}

impl ExclusionArea {
    fn exclude(&mut self, range: (i64, i64)) {
        let (range_start, range_end) = range;

        let mut inside_min: Option<i64> = None;
        let mut inside_max: Option<i64> = None;
        let mut outside: Vec<(i64, i64)> = vec![];

        let mut size = 0;
        for (excl_start, excl_end) in self.exclusions.iter() {
            let is_inside = (range_start >= *excl_start && range_start <= *excl_end)
                || (range_end >= *excl_start && range_end <= *excl_end)
                || (range_start <= *excl_start && range_end >= *excl_end);
            if is_inside {
                let min = *excl_start.min(&range_start);
                let max = *excl_end.max(&range_end);

                inside_min = match inside_min {
                    None => Some(min),
                    Some(existing) => Some(existing.min(min)),
                };
                inside_max = match inside_max {
                    None => Some(max),
                    Some(existing) => Some(existing.max(max)),
                };
            } else {
                size += (*excl_end - *excl_start) + 1;
                outside.push((*excl_start, *excl_end));
            }
        }

        if inside_min.is_some() && inside_max.is_some() {
            let mx = inside_max.unwrap();
            let mn = inside_min.unwrap();
            size += (mx - mn) + 1;
            outside.push((inside_min.unwrap(), inside_max.unwrap()));
        } else {
            size += (range.1 - range.0) + 1;
            outside.push(range);
        }
        self.exclusion_size = size;
        self.exclusions = outside;
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parse() {
        let input = include_str!("input.example.txt");
        let m = input.parse::<Map>();
        assert_eq!(true, m.is_ok());
    }

    #[test]
    fn test_part1() {
        let input = include_str!("input.example.txt");
        assert_eq!(26, part1(input, 10));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("input.example.txt");
        assert_eq!(56000011, part2(input, (20, 20).into()));
    }

    #[test]
    fn test_y_border_points() {
        // ..B..
        // .#.#.
        // #.S.#
        // .#.#.
        // ..#..

        let sensor = Sensor {
            pos: (2, 2).into(),
            closest_beacon: (2, 4).into(),
        };
        assert_eq!(
            sensor.y_border_points(0),
            Some(((2, 0).into(), (2, 0).into(),))
        );
        assert_eq!(
            sensor.y_border_points(1),
            Some(((1, 1).into(), (3, 1).into(),))
        );
        assert_eq!(
            sensor.y_border_points(2),
            Some(((0, 2).into(), (4, 2).into(),))
        );
    }

    #[test]
    fn test_new_exclusion() {
        let mut exclusion_area = ExclusionArea::default();
        exclusion_area.exclude((1, 1));

        assert_eq!(
            exclusion_area,
            ExclusionArea {
                exclusion_size: 1,
                exclusions: vec![(1, 1)],
            }
        )
    }

    #[test]
    fn test_exclusion_areas_2() {
        let mut existing_area = ExclusionArea {
            exclusions: vec![(0, 10), (20, 90)],
            exclusion_size: 82,
        };
        let exclusion = (10, 21);
        existing_area.exclude(exclusion);

        assert_eq!(
            existing_area,
            ExclusionArea {
                exclusions: vec![(0, 90)],
                exclusion_size: 91
            }
        )
    }

    #[test]
    fn test_exclusion_areas_3() {
        let mut existing_area = ExclusionArea {
            exclusions: vec![(0, 5), (7, 7), (9, 9)],
            exclusion_size: 8,
        };
        let exclusion = (0, 100);
        existing_area.exclude(exclusion);

        assert_eq!(
            existing_area,
            ExclusionArea {
                exclusions: vec![(0, 100)],
                exclusion_size: 101,
            }
        )
    }

    #[test]
    fn test_exclusion_areas_outside() {
        let mut existing_area = ExclusionArea {
            exclusions: vec![(0, 5), (7, 7), (9, 9)],
            exclusion_size: 8,
        };
        let exclusion = (90, 100);
        existing_area.exclude(exclusion);

        assert_eq!(
            existing_area,
            ExclusionArea {
                exclusions: vec![(0, 5), (7, 7), (9, 9), (90, 100),],
                exclusion_size: 19
            }
        )
    }

    #[test]
    fn test_exclusion_one_of_many() {
        let mut existing_area = ExclusionArea {
            exclusions: vec![(0, 5), (7, 7), (9, 9)],
            exclusion_size: 8,
        };
        let exclusion = (10, 10);
        existing_area.exclude(exclusion);

        assert_eq!(
            existing_area,
            ExclusionArea {
                exclusions: vec![(0, 5), (7, 7), (9, 9), (10, 10),],
                exclusion_size: 9
            }
        );
    }
}
