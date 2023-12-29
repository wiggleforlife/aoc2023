use crate::prelude::*;
use std::fs::metadata;
use MapNumber::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Entry {
    source: u32,
    dest: u32,
    range: u32,
}

static MAP_NUMBERS: [MapNumber; 8] =
    [Seed, Soil, Fertiliser, Water, Light, Temperature, Humidity, Location];

#[derive(Debug, PartialEq)]
enum MapNumber {
    Seed,
    Soil,
    Fertiliser,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

/*#[derive(Clone, Copy, Debug, PartialEq)]
struct Seed {
    soil: u32,
    fertiliser: u32,
    water: u32,
    light: u32,
    temperature: u32,
    humidity: u32,
    location: u32,
}

impl Seed {
    fn new() -> Self {
        Self {
            soil: 0,
            fertiliser: 0,
            water: 0,
            light: 0,
            temperature: 0,
            humidity: 0,
            location: 0,
        }
    }

    fn get(self, num: &MapNumber) -> u32 {
        match num {
            Seed => 0,
            Soil => self.soil,
            Fertiliser => self.fertiliser,
            Water => self.water,
            Light => self.light,
            Temperature => self.temperature,
            Humidity => self.humidity,
            Location => self.location,
        }
    }

    fn set(&mut self, mn: &MapNumber, num: u32) {
        match mn {
            Seed => (),
            Soil => self.soil = num,
            Fertiliser => self.fertiliser = num,
            Water => self.water = num,
            Light => self.light = num,
            Temperature => self.temperature = num,
            Humidity => self.humidity = num,
            Location => self.location = num,
        }
    }

    fn with(mut self, mn: &MapNumber, num: u32) -> Self {
        let mut seed = self;
        match mn {
            Seed => (),
            Soil => seed.soil = num,
            Fertiliser => seed.fertiliser = num,
            Water => seed.water = num,
            Light => seed.light = num,
            Temperature => seed.temperature = num,
            Humidity => seed.humidity = num,
            Location => seed.location = num,
        }

        seed
    }
}*/

#[derive(Clone, Copy, Debug, PartialEq)]
struct Seed {
    first: u32,
    second: u32,
}

impl Seed {
    fn new() -> Self {
        Self { first: 0, second: 0 }
    }
}

pub fn day_five(is_part_two: bool) -> String {
    let input = read_input("in/5-1.txt");
    let mut sections: Vec<Vec<_>> =
        split_vec_to_string(split_to_string(input, "\n\n"), "\n");

    let mut requested_seeds = split_parse::<u32>(
        sections.remove(0).remove(0).replace("seeds: ", ""),
        " ",
    );

    let mut maps: Vec<Vec<_>> = sections
        .iter()
        .map(|m| {
            m.iter()
                .map(|e| {
                    if Regex::new(r"[^\d ]").unwrap().is_match(e) {
                        return None;
                    }
                    let nums = split_parse::<u32>(e.clone(), " ");
                    Some(Entry {
                        source: nums[1],
                        dest: nums[0],
                        range: nums[2],
                    })
                })
                .filter(|o| o.is_some())
                .map(|o| o.unwrap())
                .collect()
        })
        .collect();

    let len = get_highest_num(maps[0].clone());
    println!("{}", len);
    let mut all_seeds = Vec::new();
    all_seeds.resize(len, Seed::new());
    /*for i in 0..len {
        all_seeds.push(Seed::new()); //FIXME hangs here - creating 429b*7 32-bit 0s
    }*/

    for mn in 0..maps.len() {
        println!("map {:?}", mn);
        let map = maps[mn].clone();
        let mn1 = &MAP_NUMBERS[mn + 1];
        let mn = &MAP_NUMBERS[mn];

        if mn != &MapNumber::Seed {
            for i in 0..all_seeds.len() {
                //let num = all_seeds[i].get(mn).clone();
                let num = all_seeds[i].first.clone();
                //all_seeds[i].set(mn1, num);
                all_seeds[i].second = num;
            }
        }
        println!("seed numbers set");
        //println!("\n\n{:?}", all_seeds);

        for entry in map {
            let mut source_range: Vec<_> =
                (entry.source..entry.source - 1 + entry.range).collect();
            source_range.push(entry.source - 1 + entry.range);
            let mut dest_range: Vec<_> =
                (entry.dest..entry.dest - 1 + entry.range).collect(); //FIXME attempted to subtract with overflow
            dest_range.push(entry.dest - 1 + entry.range);

            for i in 0..entry.range as usize {
                let source_i = i as u32 + source_range[0];
                let seeds_i = if mn != &Seed {
                    all_seeds
                        .clone()
                        .iter()
                        .position(|s| {
                            //println!("{}", s.get(mn));
                            //s.get(mn) == source_i
                            s.first == source_i
                        })
                        .unwrap()
                } else {
                    source_i as usize
                };
                //all_seeds[seeds_i] = all_seeds[seeds_i].with(mn1, dest_range[i]);
                all_seeds[seeds_i].second = dest_range[i];
            }
        }
        println!("map {:?} done", mn);
    }

    let mut locations = vec![];
    for seed in requested_seeds {
        locations.push(all_seeds[seed as usize].second);
    }
    locations.sort();

    locations.first().unwrap().to_string()
}

//FIXME? this returns the highest 32-bit number
fn get_highest_num(sts_map: Vec<Entry>) -> usize {
    let mut max_entry_nums = vec![];

    for entry in sts_map {
        let max_nums =
            (entry.source + (entry.range - 1), entry.dest + (entry.range - 1));

        max_entry_nums.push(max_nums.0.max(max_nums.1))
    }

    max_entry_nums.sort();

    *max_entry_nums.last().unwrap() as usize
}
