use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

#[derive(Debug, Clone)]
struct AlmanacMap {
    source_category: i64,
    destination_category: i64,
    distance: i64,
}

fn parse_almanac_map(line: &str) -> AlmanacMap {
    let unparsed = line.split(" ").collect::<Vec<&str>>();
    let source_category = unparsed[1];
    let destination_category = unparsed[0];
    let distance = unparsed[2];
    AlmanacMap {
        source_category: source_category.parse::<i64>().unwrap(),
        destination_category: destination_category.parse::<i64>().unwrap(),
        distance: distance.parse::<i64>().unwrap(),
    }
}

fn parse_almanac_list(lines: Vec<&str>) -> Vec<Vec<AlmanacMap>> {
    let mut categories: Vec<Vec<AlmanacMap>> = vec![];
    for line in lines[1..].iter() {
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            categories.push(Vec::new());
            continue;
        } 

        let almanac_map = parse_almanac_map(line);

        categories.last_mut().unwrap().push(almanac_map);
    }
    categories
}

fn get_part_1_seeds(lines: &str) -> Vec<i64> {
    let str_lines = lines.lines().collect::<Vec<&str>>();
    let input_seeds = str_lines[0].split(" ").collect::<Vec<&str>>();
    let mut seeds: Vec<i64> = Vec::new();
    for seed in input_seeds[1..].iter(){
        seeds.push(seed.parse::<i64>().unwrap());
    }
    seeds
}

fn get_part_2_seeds(lines: &str) -> Vec<AlmanacMap> {
    let str_lines = lines.lines().collect::<Vec<&str>>();
    let input_seeds = str_lines[0].split(" ").collect::<Vec<&str>>();
    let mut seeds: Vec<AlmanacMap> = Vec::new();
    println!("{:?}", input_seeds);
    for seed_range in input_seeds[1..].chunks(2) {
        let string_start = seed_range[0];
        let string_length = seed_range[1];
        let start = string_start.parse::<i64>().unwrap();
        let length = string_length.parse::<i64>().unwrap();
        seeds.push(AlmanacMap {
            source_category: start,
            distance: length,
            destination_category: start,
        });
    }
    seeds
}

fn part_1() {
    let lines = read_to_string("input.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    let input_seeds = get_part_1_seeds(str_lines[0]);
    let mut categories: Vec<Vec<AlmanacMap>> = parse_almanac_list(str_lines);

    let mut paths: Vec<i64> = Vec::new();
    let mut known_paths: Vec<HashMap<i64, i64>> = vec![];
    for seed in input_seeds.iter(){
        let mut source = *seed;
        let mut routes: Vec<i64> = Vec::new();
        for (j, category) in categories.iter_mut().enumerate() {
            let known_path = known_paths.get(j);
            if known_path.is_none() {
                known_paths.push(HashMap::new());
            }
            let known_path = known_paths.get_mut(j).unwrap();
            if known_path.contains_key(&source) {
                source = known_path.get(&source).unwrap().clone();
                break;
            }

            let mut destination: Option<i64> = None;
            for almanac_map in category.iter_mut() {
                if source >= almanac_map.source_category && source < almanac_map.source_category + almanac_map.distance {
                    destination = Some(almanac_map.destination_category + (source - almanac_map.source_category));
                    break;
                }
            }
            if destination.is_none() {
                destination = Some(source);
            }
            routes.push(source);
            source = destination.unwrap();
        }
        for (i, route) in routes.iter().enumerate() {
            known_paths[i].insert(*route, source);
        }

        paths.push(source);
    }

    println!("part 1 result {:?}", paths.iter().min().unwrap());
}   

fn hash_almanac_map(almanac_map: &AlmanacMap) -> String {
    format!("{}{}", almanac_map.destination_category, almanac_map.distance)
}

fn bfs(sources: Vec<AlmanacMap>, 
       mut category_destinations: VecDeque<Vec<AlmanacMap>>, 
      ) -> Vec<i64> {
    if category_destinations.is_empty() {
        return sources.iter().map(|x| x.destination_category).collect::<Vec<i64>>();
    }
    let destinations = category_destinations.pop_front().unwrap();
    let mut next_sources: Vec<AlmanacMap> = Vec::new();
    let mut mut_sources = sources.clone();
    let mut visited: HashMap<String, bool> = HashMap::new();
    while !mut_sources.is_empty() {
        let source = mut_sources.pop().unwrap();
        let hashed_source = hash_almanac_map(&source.clone());
        if visited.contains_key(&hashed_source.clone()) {
            continue;
        }
        
        let AlmanacMap{source_category: _, destination_category, distance} = source;
        let mut found =  false;
        for destination in  destinations.iter() {
            if destination_category <= destination.source_category && destination_category + distance >= destination.source_category {
                let mut new_distance = 0;
                if destination_category + distance >= destination.source_category + destination.distance {
                    new_distance = destination.distance;
                    
                } else if destination_category + distance < destination.source_category + destination.distance{
                    new_distance = destination_category + distance - destination.source_category;
                }
                if new_distance == 0 {
                    continue;
                }
                let new_destination = AlmanacMap {
                    source_category: destination.source_category,
                    destination_category: destination.destination_category,
                    distance: new_distance,
                };
                next_sources.push(new_destination);
                if destination_category < destination.source_category {
                    let new_source = AlmanacMap {
                        source_category: 0,
                        destination_category,
                        distance: destination.source_category - destination_category,
                    };
                    mut_sources.push(new_source);
                }
                found = true;
                continue;
            }

            // if the destination category is greater than the start, but it's distance falls
            // within the range split it into two paths
            if destination_category >= destination.source_category && destination_category <= destination.source_category + destination.distance {
                let mut new_distance = 0;
                if destination_category + distance <= destination.source_category + destination.distance {
                    new_distance = distance;
                } else if destination_category + distance > destination.source_category + destination.distance{
                    new_distance = destination_category + distance - destination.source_category;
                }
                let new_destination = AlmanacMap {
                    source_category: destination_category,
                    destination_category: destination.destination_category + destination_category - destination.source_category,
                    distance: new_distance,
                };

                if destination_category + distance > destination.source_category + destination.distance {
                    let new_distance = destination_category + distance - destination.source_category - destination.distance;
                    let new_source = AlmanacMap {
                        source_category: 0,
                        destination_category: destination.source_category + destination.distance + 1,
                        distance: new_distance,
                    };
                    mut_sources.push(new_source);
                }

                found = true;
                next_sources.push(new_destination);
                continue;
            }
        }
        if !found {
            next_sources.push(source);
        }
        visited.insert(hashed_source.clone(), true);
    }
    return bfs(next_sources, category_destinations);
    
}

fn part_2() {
    // println!("Hello, world!");
    let lines = read_to_string("input.txt")
        .unwrap();
    let str_lines = lines.lines().collect::<Vec<&str>>();
    let input_seeds = get_part_2_seeds(str_lines[0]);

    let categories: Vec<Vec<AlmanacMap>> = parse_almanac_list(str_lines);
    let paths = bfs(input_seeds, categories.to_vec().into_iter().collect::<VecDeque<Vec<AlmanacMap>>>());

    println!("part 2 {:?}", paths.iter().filter(|x| **x > 1).min().unwrap_or(&-3));
}


fn main() {
    part_1();
    part_2();

}
