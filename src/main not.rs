use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let map = get_names();
    let map = get_traffic(map);
    let map = get_quality(map);
    for entry in map {
        if entry.1.0 != 0 || entry.1.1 == 0 {
            continue;
        }
        println!("{}, {}, {}", entry.0, entry.1.0, entry.1.1);
    }
}

fn get_names<>() -> HashMap<String, (u64, u64, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\traffic.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    let mut map: HashMap<String, (u64, u64, u64, u64)> = HashMap::new(); // Traffic Count, Road Quality, # of traffic entries, # of quality entries
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let s = s.next().unwrap();
        if !map.contains_key(s) {
            map.insert(String::from(s), (0, 0, 1, 0));
        } else {
            let mut tuple = map.get(s).unwrap().clone();
            tuple.2 += 1;
            map.insert(String::from(s), tuple);
        }
    }
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\quality.txt").unwrap()).unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let s = s.next().unwrap();
        if !map.contains_key(s) {
            map.insert(String::from(s), (0, 0, 0, 1));
        } else {
            let mut tuple = map.get(s).unwrap().clone();
            tuple.3 += 1;
            map.insert(String::from(s), tuple);
        }
    }
    map
}

fn get_traffic(mut map: HashMap<String, (u64, u64, u64, u64)>) -> HashMap<String, (u64, u64, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\traffic.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let name = s.next().unwrap();
        let name = String::from(name);
        let next = s.next();
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        let count = u64::from_str(next);
        if count.is_err() {
            continue;
        }
        let count = count.unwrap();
        let mut tuple = *map.get(&name.clone()).unwrap();
        let new_traffic = count / map.get(&String::from(name.clone())).unwrap().2;
        tuple.0 = new_traffic;
        map.insert(name, tuple);
    }
    map
}

fn get_quality(mut map: HashMap<String, (u64, u64, u64, u64)>) -> HashMap<String, (u64, u64, u64, u64)> {
    let file = String::from_utf8(std::fs::read("E:\\SJSU\\CS\\131\\finalproj\\quality.txt").unwrap()).unwrap();
    let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
    let split = regex.split(file.as_str());
    for s in split {
        let regex = Regex::new(r"(?m),").unwrap();
        let mut s = regex.split(s).into_iter();
        let name = s.next().unwrap();
        let name = String::from(name);
        let next = s.next();
        if next.is_none() {
            continue;
        }
        let next = next.unwrap();
        let quality = u64::from_str(next);
        if quality.is_err() {
            continue;
        }
        let quality = quality.unwrap();
        let mut tuple = *map.get(&name.clone()).unwrap();
        let new_quality = quality / map.get(&String::from(name.clone())).unwrap().3;
        tuple.1 = new_quality;
        map.insert(name, tuple);
    }
    map
}