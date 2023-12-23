use std::{collections::btree_map::Range, num::ParseIntError, str::FromStr};

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{alpha0, digit0, line_ending, space1},
    combinator::map_res,
    error::{FromExternalError, VerboseError},
    multi::{separated_list0, separated_list1},
    sequence::{pair, separated_pair, terminated},
    Finish, IResult,
};

use anyhow::{format_err, Error};

fn main() -> Result<(), Error> {
    let input = include_str!("../input.txt");

    println!("Part 1 result: {:?}", part1(input));

    println!("Part 2 result: {:?}", part2(input));

    Ok(())
}

fn part1(input: &str) -> Result<usize, Error> {
    let map: PlantingMap = input.parse()?;
    Ok(*map.seed_locations().iter().min().unwrap())
}

fn part2(input: &str) -> Result<usize, Error> {
    let map: PlantingMap = input.parse()?;
    let (loc, seed) = map.lowest_seed_from_location();
    println!("Found {seed} which maps to {loc}");
    Ok(loc)
}

#[derive(Debug, Copy, Clone)]
struct MapRange {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl MapRange {
    fn in_range(&self, source: usize) -> bool {
        self.source_start <= source && self.source_start + self.len > source
    }

    fn map(&self, source: usize) -> Option<usize> {
        if !self.in_range(source) {
            None
        } else {
            Some(source - self.source_start + self.dest_start)
        }
    }

    fn unmap(&self, dest: usize) -> Option<usize> {
        if self.dest_start <= dest && self.dest_start + self.len > dest {
            Some(dest - self.dest_start + self.source_start)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Mapping {
    from: String,
    to: String,
    remapped: Vec<MapRange>,
}

impl Mapping {
    fn map(&self, source: usize) -> usize {
        for map in self.remapped.iter() {
            if let Some(dest) = map.map(source) {
                return dest;
            }
        }
        source
    }

    fn unmap(&self, sink: usize) -> usize {
        for map in self.remapped.iter() {
            if let Some(unmap) = map.unmap(sink) {
                return unmap;
            }
        }
        sink
    }
}

struct PlantingMap {
    seeds: Vec<usize>,
    seed_soil: Mapping,
    soil_fertilizer: Mapping,
    fertilizer_water: Mapping,
    water_light: Mapping,
    light_temp: Mapping,
    temp_humidity: Mapping,
    humidity_location: Mapping,
}

impl PlantingMap {
    fn seed_location(&self, seed: usize) -> usize {
        let soil = self.seed_soil.map(seed);
        let fertilizer = self.soil_fertilizer.map(soil);
        let water = self.fertilizer_water.map(fertilizer);
        let light = self.water_light.map(water);
        let temp = self.light_temp.map(light);
        let humidity = self.temp_humidity.map(temp);
        self.humidity_location.map(humidity)
    }

    fn location_seed(&self, location: usize) -> usize {
        let humidity = self.humidity_location.unmap(location);
        let temp = self.temp_humidity.unmap(humidity);
        let light = self.light_temp.unmap(temp);
        let water = self.water_light.unmap(light);
        let fertilizer = self.fertilizer_water.unmap(water);
        let soil = self.soil_fertilizer.unmap(fertilizer);
        self.seed_soil.unmap(soil)
    }

    fn seed_locations(&self) -> Vec<usize> {
        self.seeds
            .clone()
            .into_iter()
            .map(|x| self.seed_location(x))
            .collect()
    }

    fn seed_locations_range_min(&self) -> usize {
        let mut min = std::usize::MAX;
        for range_spec in self.seeds.as_slice().chunks(2) {
            println!("{} seeds to consider in this range", range_spec[1]);
            for seed in range_spec[0]..=range_spec[0] + range_spec[1] {
                let loc = self.seed_location(seed);
                if loc < min {
                    min = loc;
                }
            }
            println!("{} considered in this range: min now {min}", range_spec[1]);
        }
        min
    }

    fn lowest_seed_from_location(&self) -> (usize, usize) {
        let mut seed_ranges = Vec::new();
        for range_spec in self.seeds.as_slice().chunks(2) {
            seed_ranges.push(range_spec[0]..range_spec[0] + range_spec[1]);
        }
        for location in 0..usize::MAX {
            if location % 1_000_000 == 0 {
                println!("At {location}, still no seeds");
            }
            let seed = self.location_seed(location);
            if seed_ranges.iter().any(|x| x.contains(&seed)) {
                return (location, seed);
            }
        }
        (usize::MAX, usize::MAX)
    }
}

impl FromStr for PlantingMap {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn seeds(input: &str) -> IResult<&str, Vec<usize>, VerboseError<&str>> {
            let (i, _) = tag("seeds: ")(input)?;
            let (i, numbers) =
                separated_list0(space1, map_res(digit0, |s: &str| s.parse::<usize>()))(i)?;
            Ok((i, numbers))
        }
        fn map_range(input: &str) -> IResult<&str, MapRange, VerboseError<&str>> {
            let (i, nums) =
                separated_list1(space1, map_res(digit0, |s: &str| s.parse::<usize>()))(input)?;
            if nums.len() != 3 {
                return Err(nom::Err::Error(VerboseError::from_external_error(
                    i,
                    nom::error::ErrorKind::MapRes,
                    format_err!("Expected 3 numbers"),
                )));
            }
            Ok((
                i,
                MapRange {
                    dest_start: nums[0],
                    source_start: nums[1],
                    len: nums[2],
                },
            ))
        }
        fn mapping(input: &str) -> IResult<&str, Mapping, VerboseError<&str>> {
            let (i, types) = terminated(
                separated_pair(alpha0, tag("-to-"), is_not(" ")),
                tag(" map:\n"),
            )(input)?;
            let (i, ranges) = separated_list1(line_ending, map_range)(i)?;
            println!(
                "found {} to {} mapping with {} remapped ranges: {ranges:#?}",
                types.0,
                types.1,
                ranges.len()
            );
            Ok((
                i,
                Mapping {
                    from: types.0.to_owned(),
                    to: types.1.to_owned(),
                    remapped: ranges,
                },
            ))
        }
        fn blankline(input: &str) -> IResult<&str, (), VerboseError<&str>> {
            let (rest, _) = pair(line_ending, line_ending)(input)?;
            Ok((rest, ()))
        }
        match separated_pair(seeds, blankline, separated_list1(blankline, mapping))(s).finish() {
            Ok((_i, (seeds, mappings))) => {
                if mappings.len() != 7 {
                    return Err(format_err!("Not enough mappings: {mappings:#?}"));
                }
                Ok(Self {
                    seeds,
                    seed_soil: mappings[0].clone(),
                    soil_fertilizer: mappings[1].clone(),
                    fertilizer_water: mappings[2].clone(),
                    water_light: mappings[3].clone(),
                    light_temp: mappings[4].clone(),
                    temp_humidity: mappings[5].clone(),
                    humidity_location: mappings[6].clone(),
                })
            }
            Err(prob) => Err(format_err!("Issue parsing: {prob:?}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EX_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn parse_input() {
        let map: PlantingMap = EX_INPUT.parse().expect("parses okay");
        assert_eq!(4, map.seeds.len());

        assert_eq!(98, map.seed_soil.remapped[0].source_start);
    }

    #[test]
    fn ex_part1() {
        assert_eq!(35, part1(EX_INPUT).unwrap());
    }

    #[test]
    fn ex_part2() {
        assert_eq!(46, part2(EX_INPUT).unwrap());
    }
}
