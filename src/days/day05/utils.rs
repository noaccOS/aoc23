pub fn read_input() -> &'static str {
    include_str!("input")
}

use std::ops::Range;

pub struct Almanac {
    pub seeds: Vec<usize>,
    seed_to_soil: Vec<(Range<usize>, Range<usize>)>,
    soil_to_fertilizer: Vec<(Range<usize>, Range<usize>)>,
    fertilizer_to_water: Vec<(Range<usize>, Range<usize>)>,
    water_to_light: Vec<(Range<usize>, Range<usize>)>,
    light_to_temperature: Vec<(Range<usize>, Range<usize>)>,
    temperature_to_humidity: Vec<(Range<usize>, Range<usize>)>,
    humidity_to_location: Vec<(Range<usize>, Range<usize>)>,
}

impl Almanac {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let seeds: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();
        let seed_to_soil = Self::read_new_section(&mut lines);
        let soil_to_fertilizer = Self::read_new_section(&mut lines);
        let fertilizer_to_water = Self::read_new_section(&mut lines);
        let water_to_light = Self::read_new_section(&mut lines);
        let light_to_temperature = Self::read_new_section(&mut lines);
        let temperature_to_humidity = Self::read_new_section(&mut lines);
        let humidity_to_location = Self::read_new_section(&mut lines);

        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn read_new_section(
        lines: &mut impl Iterator<Item = impl AsRef<str>>,
    ) -> Vec<(Range<usize>, Range<usize>)> {
        let mut first = lines.next().unwrap();
        while first.as_ref() == "" {
            first = lines.next().unwrap();
        }
        // after loop, lines is at something-to-something-map. we can start consuming the lines

        lines
            .take_while(|l| !l.as_ref().is_empty())
            .map(|l| l.as_ref().split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<_>>())
            .map(|line| {
                let to_start = line[0];
                let from_start = line[1];
                let count = line[2];
                (from_start..from_start + count, to_start..to_start + count)
            })
            .collect()
    }

    fn find_from_map(iter: &[(Range<usize>, Range<usize>)], from: usize) -> usize {
        for (from_range, to_range) in iter {
            if from_range.contains(&from) {
                let pad = from - from_range.start;
                return to_range.start + pad;
            }
        }
        from
    }

    pub fn seed_to_location(&self, seed: usize) -> usize {
        let soil = Self::find_from_map(&self.seed_to_soil, seed);
        let fertilizer = Self::find_from_map(&self.soil_to_fertilizer, soil);
        let water = Self::find_from_map(&self.fertilizer_to_water, fertilizer);
        let light = Self::find_from_map(&self.water_to_light, water);
        let temperature = Self::find_from_map(&self.light_to_temperature, light);
        let humidity = Self::find_from_map(&self.temperature_to_humidity, temperature);
        let location = Self::find_from_map(&self.humidity_to_location, humidity);
        location
    }
}
