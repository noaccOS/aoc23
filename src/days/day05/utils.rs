pub fn read_input() -> &'static str {
    include_str!("input")
}

use std::ops::Range;

pub struct Almanac {
    pub seeds: Vec<usize>,
    pub seeds_ranges: Vec<Range<usize>>,
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
        let seeds_ranges = Self::build_seed_ranges(&seeds);
        let seed_to_soil = Self::read_new_section(&mut lines);
        let soil_to_fertilizer = Self::read_new_section(&mut lines);
        let fertilizer_to_water = Self::read_new_section(&mut lines);
        let water_to_light = Self::read_new_section(&mut lines);
        let light_to_temperature = Self::read_new_section(&mut lines);
        let temperature_to_humidity = Self::read_new_section(&mut lines);
        let humidity_to_location = Self::read_new_section(&mut lines);

        Self {
            seeds,
            seeds_ranges,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    fn build_seed_ranges(seed_ranges_input: &Vec<usize>) -> Vec<Range<usize>> {
        let mut res: Vec<_> = seed_ranges_input
            .chunks(2)
            .map(|a| {
                let [seed, length] = a.try_into().unwrap();
                seed..seed + length
            })
            .collect();
        res.sort_by(|a, b| a.start.cmp(&b.start));
        res
    }

    fn read_new_section(
        lines: &mut impl Iterator<Item = impl AsRef<str>>,
    ) -> Vec<(Range<usize>, Range<usize>)> {
        let mut first = lines.next().unwrap();
        while first.as_ref() == "" {
            first = lines.next().unwrap();
        }
        // after loop, lines is at something-to-something-map. we can start consuming the lines

        let mut section: Vec<_> = lines
            .take_while(|l| !l.as_ref().is_empty())
            .map(|l| {
                l.as_ref()
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .map(|line| {
                let to_start = line[0];
                let from_start = line[1];
                let count = line[2];
                (from_start..from_start + count, to_start..to_start + count)
            })
            .collect();

        section.sort_by(|(a, to_a), (b, to_b)| a.start.cmp(&b.start));
        Self::fill_section_with_missing(&section)
        //section
    }

    fn fill_section_with_missing(
        section: &Vec<(Range<usize>, Range<usize>)>,
    ) -> Vec<(Range<usize>, Range<usize>)> {
        let mut res = Vec::with_capacity(section.len() * 2 - 1);
        let first = section[0].clone();
        let (from, to) = first.clone();
        if from.start != 0 {
            let zero_to_start = 0..from.start;
            res.push((zero_to_start.clone(), zero_to_start));
        }
        res.push(first);
        for val in section[..].windows(2) {
            let first = &val[0];
            let second = &val[1];
            let start = first.0.end;
            let end = second.0.start;
            if start != end {
                let missing = start..end;
                res.push((missing.clone(), missing));
            }
            res.push(second.clone());
        }
        let (last_from, _last_to) = res.last().clone().unwrap();
        let up_to_infinity = last_from.end..usize::MAX;
        res.push((up_to_infinity.clone(), up_to_infinity));
        res
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

    pub fn seed_range_to_minimum_location(&self) -> usize {
        // println!("temeperature_to_humb")
        let mut location_ranges: Vec<_> = self
            .humidity_to_location
            .iter()
            .map(|(_, location)| location)
            .collect();
        location_ranges.sort_by(|a, b| a.start.cmp(&b.start));
        let min_seeds_iter = location_ranges
        .iter()
        .filter_map(|location_range| {
            self.location_to_humidity(&location_range)
                .and_then(|(seed, pad)| {
                    println!(
                        "found seed {} with pad {} for range {:?}",
                        seed, pad, location_range
                    );
                    Some(seed)
                })
        });

        // let min_seed =
        //     .next()
        //     .unwrap();
        min_seeds_iter.map(|seed| self.seed_to_location(seed)).min().unwrap()
    }

    fn location_to_humidity(&self, range: &Range<usize>) -> Option<(usize, usize)> {
        let next_ranges = self
            .humidity_to_location
            .iter()
            .map(|(from, to)| (from, to));

        let intersections = Self::intersect_ranges(range, next_ranges, &0usize);

        for (intersection, pad) in intersections {
            println!(
                "location_to_humidity: range {:?}, intersection: {:?}",
                range, intersection
            );
            let next = self.temperature_to_light(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "location_to_humidity: did not find anything for range {:?}",
            range
        );
        None
    }

    fn humidity_to_temperature(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        let next_ranges = self
            .temperature_to_humidity
            .iter()
            .map(|(from, to)| (from, to));

        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            println!(
                "humidity_to_temperature: range {:?}, intersection: {:?}",
                range, intersection
            );
            let next = self.temperature_to_light(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "humidity_to_temperature: did not find anything for range {:?}",
            range
        );
        None
    }

    fn temperature_to_light(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        println!("enter temperature_to_light");

        let next_ranges = self
            .light_to_temperature
            .iter()
            .map(|(from, to)| (from, to));
        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            let next = self.light_to_water(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "temperature_to_light: did not find anything for range {:?}",
            range
        );
        None
    }

    fn light_to_water(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        let next_ranges = self.water_to_light.iter().map(|(from, to)| (from, to));
        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            let next = self.water_to_fertilizer(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "light_to_water: did not find anything for range {:?}",
            range
        );
        None
    }

    fn water_to_fertilizer(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        let next_ranges = self.fertilizer_to_water.iter().map(|(from, to)| (from, to));
        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            let next = self.fertilizer_to_soil(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "water_to_fertilizer: did not find anything for range {:?}",
            range
        );
        None
    }

    fn fertilizer_to_soil(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        let next_ranges = self.soil_to_fertilizer.iter().map(|(from, to)| (from, to));
        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            let next = self.soil_to_seed(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!(
            "fertilizer_to_soil: did not find anything for range {:?}",
            range
        );
        None
    }

    fn soil_to_seed(&self, range: &Range<usize>, pad: &usize) -> Option<(usize, usize)> {
        let next_ranges = self.seed_to_soil.iter().map(|(from, to)| (from, to));
        let intersections = Self::intersect_ranges(range, next_ranges, pad);

        for (intersection, pad) in intersections {
            let next = self.find_any_seed_in(&intersection, &pad);
            if next.is_some() {
                return next;
            }
        }
        println!("soil_to_seed: did not find anything for range {:?}", range);
        None
    }

    fn find_any_seed_in<'a>(
        &'a self,
        seed_range: &'a Range<usize>,
        pad: &usize,
    ) -> Option<(usize, usize)> {
        Self::intersect_ranges_simple(seed_range, self.seeds_ranges.iter(), pad)
            .next()
            .and_then(|(intersection, pad)| Some((intersection.start, pad)))
    }

    fn intersect_ranges<'a>(
        range: &'a Range<usize>,
        next_ranges: impl Iterator<Item = (&'a Range<usize>, &'a Range<usize>)> + 'a,
        pad: &'a usize,
    ) -> impl Iterator<Item = (Range<usize>, usize)> + 'a {
        // let max_len = next_ranges.len();
        next_ranges
            .filter_map(move |(to, from)| {
                if from.start <= range.start && from.end >= range.end {
                    // from fully contains range, return the intersection = range
                    let range_begin_pad = range.start - from.start;
                    let to_begin = to.start + range_begin_pad;
                    let range_end_pad = from.end - range.end;
                    let to_end = to.end - range_end_pad;

                    let _from_range = range.clone();
                    let to_range = to_begin..to_end;
                    Some((to_range, *pad))
                } else if from.start >= range.start && from.end <= range.end {
                    // range fully contains from, = from
                    let pad = pad + from.start - range.start;
                    let _from_range = from.clone();
                    let to_range = to.clone();
                    Some((to_range, pad))
                } else if range.contains(&from.start) {
                    // starts in the range, ends after. returns intersection
                    let pad = pad + from.start - range.start;
                    let dim = range.end - from.start;
                    let to_end = to.start + dim;

                    let _from_range = from.start..range.end;
                    let to_range = to.start..to_end;
                    Some((to_range, pad))
                } else if range.contains(&(from.end - 1)) {
                    // starts before range, ends in range
                    let dim = from.end - range.start;
                    let to_start = to.end - dim;

                    let _from_range = range.start..from.end;
                    let to_range = to_start..to.end;
                    Some((to_range, *pad))
                } else {
                    // no intersection
                    None
                }
            })
            .filter(|(range, _pad)| range.len() > 0)
    }

    fn intersect_ranges_simple<'a>(
        range: &'a Range<usize>,
        next_ranges: impl Iterator<Item = &'a Range<usize>> + 'a,
        pad: &'a usize,
    ) -> impl Iterator<Item = (Range<usize>, usize)> + 'a {
        // let max_len = next_ranges.len();
        next_ranges
            .filter_map(move |next| {
                if next.start <= range.start && next.end >= range.end {
                    // next fully contains range, return the intersection = range
                    let out = range.clone();
                    Some((out, *pad))
                } else if next.start >= range.start && next.end <= range.end {
                    // range fully contains next, = next
                    let pad = pad + next.start - range.start;
                    let out = next.clone();
                    Some((out, pad))
                } else if range.contains(&next.start) {
                    // starts in the range, ends after. returns intersection
                    let pad = pad + next.start - range.start;
                    let out = next.start..range.end;
                    Some((out, pad))
                } else if range.contains(&(next.end - 1)) {
                    // starts before range, ends in range
                    let out = range.start..next.end;
                    Some((out, *pad))
                } else {
                    // no intersection
                    None
                }
            })
            .filter(|(range, _pad)| range.len() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fill_gaps() {
        let orig = vec![(0..5, 20..25), (7..10, 15..18)];
        let dest = Almanac::fill_section_with_missing(&orig);
        assert_eq!(orig[0], dest[0]);
        assert_eq!(orig[1], dest[2]);
        assert_eq!(dest[1], (5..7, 5..7));
    }

    #[test]
    fn soil_to_seed_test() {
        let sample = include_str!("sample_b");
        let almanac = Almanac::from_input(sample);
        let range: Range<usize> = 52..58;
        assert_eq!(almanac.soil_to_seed(&range, &0), Some((55, 5)))
    }

    #[test]
    fn fertilizer_to_soil_test() {
        let sample = include_str!("sample_b");
        let almanac = Almanac::from_input(sample);
        let range: Range<usize> = 52..58;
        assert_eq!(almanac.fertilizer_to_soil(&range, &0), Some((55, 5)))
    }
}
