advent_of_code::solution!(5);

#[derive(Debug,PartialEq,Clone,Copy)]
struct Mapping {
    dst_start: u32,
    src_start: u32,
    length: u32,
}


pub fn part_one(input: &str) -> Option<u32> {
    let items = input.split("\n\n").collect::<Vec<_>>();
    let seeds = items[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let process_mapping = |x: &str| -> Vec<Mapping> {
        x.lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let dst_start = parts.next().unwrap().parse::<u32>().unwrap();
                let src_start = parts.next().unwrap().parse::<u32>().unwrap();
                let length = parts.next().unwrap().parse::<u32>().unwrap();
                Mapping {
                    dst_start,
                    src_start,
                    length,
                }
            })
            .collect::<Vec<_>>()
    };
    let seed_to_soil = process_mapping(items[1]);
    let soil_to_fertilizer = process_mapping(items[2]);
    let fertilizer_to_water = process_mapping(items[3]);
    let water_to_light = process_mapping(items[4]);
    let light_to_temperature = process_mapping(items[5]);
    let temperature_to_humidity = process_mapping(items[6]);
    let humidity_to_location = process_mapping(items[7]);

    let mut locations: Vec<u32> = vec![0; seeds.len()];
    for (index, seed) in seeds.iter().enumerate() {
        let mut soil = 0;
        let mut mapped = false;
        for mapping in seed_to_soil.iter() {
            if *seed >= mapping.src_start
                && *seed
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                soil = mapping
                    .dst_start
                    .wrapping_add(*seed)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        // if not mapped, use the src number
        if !mapped {
            soil = *seed;
        }

        let mut fertilizer = 0;
        mapped = false;
        for mapping in soil_to_fertilizer.iter() {
            if soil >= mapping.src_start
                && soil
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                fertilizer = mapping
                    .dst_start
                    .wrapping_add(soil)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        if !mapped {
            fertilizer = soil;
        }

        let mut water = 0;
        mapped = false;
        for mapping in fertilizer_to_water.iter() {
            if fertilizer >= mapping.src_start
                && fertilizer
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                water = mapping
                    .dst_start
                    .wrapping_add(fertilizer)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        if !mapped {
            water = fertilizer;
        }

        let mut light = 0;
        mapped = false;
        for mapping in water_to_light.iter() {
            if water >= mapping.src_start
                && water
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                light = mapping
                    .dst_start
                    .wrapping_add(water)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        if !mapped {
            light = water;
        }

        let mut temperature = 0;
        mapped = false;
        for mapping in light_to_temperature.iter() {
            if light >= mapping.src_start
                && light
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                temperature = mapping
                    .dst_start
                    .wrapping_add(light)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }

        if !mapped {
            temperature = light;
        }

        let mut humidity = 0;
        mapped = false;
        for mapping in temperature_to_humidity.iter() {
            if temperature >= mapping.src_start
                && temperature
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                humidity = mapping
                    .dst_start
                    .wrapping_add(temperature)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        if !mapped {
            humidity = temperature;
        }

        let mut location = 0;
        mapped = false;
        for mapping in humidity_to_location.iter() {
            if humidity >= mapping.src_start
                && humidity
                    < mapping
                        .src_start
                        .checked_add(mapping.length)
                        .unwrap_or(u32::MAX)
            {
                location = mapping
                    .dst_start
                    .wrapping_add(humidity)
                    .wrapping_sub(mapping.src_start);
                mapped = true;
                break;
            }
        }
        if !mapped {
            location = humidity;
        }

        locations[index] = location;
        println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
    }
    // sort locations
    locations.sort();
    Some(locations[0])
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = input.split("\n\n").collect::<Vec<_>>();
    let seeds = items[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<_>>();
    let process_mapping = |x: &str| -> Vec<Mapping> {
        x.lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace();
                let dst_start = parts.next().unwrap().parse::<u32>().unwrap();
                let src_start = parts.next().unwrap().parse::<u32>().unwrap();
                let length = parts.next().unwrap().parse::<u32>().unwrap();
                Mapping {
                    dst_start,
                    src_start,
                    length,
                }
            })
            .collect::<Vec<_>>()
    };
    let seed_to_soil = process_mapping(items[1]);
    let soil_to_fertilizer = process_mapping(items[2]);
    let fertilizer_to_water = process_mapping(items[3]);
    let water_to_light = process_mapping(items[4]);
    let light_to_temperature = process_mapping(items[5]);
    let temperature_to_humidity = process_mapping(items[6]);
    let humidity_to_location = process_mapping(items[7]);

    let mut min_position = 0;

    let transform_ranges = |ranges: Vec<(u32, u32)>, mappings: &Vec<Mapping>| -> Vec<(u32, u32)> {
        let mut result = vec![];
        for range in ranges.iter() {

        }
        result
    };
    for seed in seeds.iter() {
        let seed_ranges = vec![(seed.0, seed.1)];
        let soil_ranges = transform_ranges(seed_ranges, &seed_to_soil);
        let fertilizer_ranges = transform_ranges(soil_ranges, &soil_to_fertilizer);
        let water_ranges = transform_ranges(fertilizer_ranges, &fertilizer_to_water);
        let light_ranges = transform_ranges(water_ranges, &water_to_light);
        let temperature_ranges = transform_ranges(light_ranges, &light_to_temperature);
        let humidity_ranges = transform_ranges(temperature_ranges, &temperature_to_humidity);
        let location_ranges = transform_ranges(humidity_ranges, &humidity_to_location);
        for location_range in location_ranges.iter() {
            if location_range.0 < min_position {
                min_position = location_range.0;
            }
        }
    }

    Some(min_position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
