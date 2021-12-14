use std::{collections::HashMap, hash::Hash};

pub fn template() -> String {
    "KFFNFNNBCNOBCNPFVKCP".to_string()
}

pub fn mutations<'a>(d: &'a str) -> HashMap<u16, u8> {
    let mut map = HashMap::new();
    for line in d.lines() {
        if line.len() > 3 {
            let (khi, klo) = (line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap());
            let v = line.chars().rev().nth(0).unwrap();
            let iv = v as u8;
            map.insert((khi as u16) << 8u16 | klo as u16, iv);
        }
    }
    map
}

fn main() {
    let iterations = 40;
    let mut count = [0u64; 256];
    let t = template();
    let bytes = t.as_bytes();
    for b in bytes {
        count[*b as usize] += 1;
    }
    let polymermapping = mutations(include_str!("../mutations"));

    let mut set: HashMap<u16, u64> = HashMap::new();
    for pair in bytes.windows(2) {
        let k = (pair[0] as u16) << 8 | pair[1] as u16;
        if set.contains_key(&k) {
            let a = set.get_mut(&k).unwrap();
            *a += 1;
        } else {
            set.insert(k, 1);
        }
    }

    for k in polymermapping.keys() {
        if !set.contains_key(&k) {
            set.insert(*k, 0);
        }
    }
    let mut mtmp: HashMap<u16, u64> = HashMap::new();
    for k in polymermapping.keys() {
        mtmp.insert(*k, 0);
    }
    for x in 0..iterations {
        for (k, v) in set.iter_mut() {
            let add_key_lo = polymermapping.get(&k).unwrap();
            let add_key = (k & 0xff00) | *add_key_lo as u16;
            *mtmp.get_mut(&add_key).unwrap() += *v;
            let addkey2 = (*add_key_lo as u16) << 8 | (k & 0xff);
            *mtmp.get_mut(&addkey2).unwrap() += *v;
        }

        for (k, v_to_add) in mtmp.iter_mut() {
            *set.get_mut(&k).unwrap() = *v_to_add as u64;
            *v_to_add = 0;
        }
    }

    let mut min = u64::MAX;
    let mut max = 0;
    for (k, v) in set {
        let hi = (k >> 8) as u8;
        let lo = (k & 0xff) as u8;
        count[hi as usize] += (v as u64 + 1) / 2;
        count[lo as usize] += (v as u64 + 1) / 2;
    }

    for x in count.iter().skip(65).take('Z' as usize - 'A' as usize + 1) {
        min = if *x > 0 { std::cmp::min(min, *x) } else { min };
        max = std::cmp::max(max, *x);
    }

    println!("MAX: {}, MIN: {} after {} iterations: result: {}", max, min, iterations, max - min);
}
