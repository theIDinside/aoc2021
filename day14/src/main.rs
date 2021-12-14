use std::collections::HashMap;

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

pub fn vec_reserved(template: &str, mutations: usize) -> (Vec<u8>, Vec<u8>) {
    let base_insertion_count = template.len() - 1;
    let mut total = template.len();
    let mut it = base_insertion_count;
    for _ in 0..mutations {
        total += it;
        it *= 2;
    }
    (Vec::with_capacity(total), Vec::with_capacity(total))
}

fn main() {
    let (mut src, mut dst) = vec_reserved(&template(), 10);
    let mut count = [0; 256];
    let t = template();
    let bytes = t.as_bytes();
    for b in bytes {
        src.push(*b);
        count[*b as usize] += 1;
    }
    let polymermapping = mutations(include_str!("../mutations"));
    for _ in 0..10 {
        let mut pos = 0usize;
        for pair in src.windows(2) {
            let k = (pair[0] as u16) << 8 | pair[1] as u16;
            let insert = polymermapping.get(&k).unwrap();
            dst.insert(pos, pair[0]);
            dst.insert(pos + 1, *insert);
            pos += 2;
            count[*insert as usize] += 1;
        }
        dst.insert(pos, *src.last().unwrap());
        std::mem::swap(&mut src, &mut dst);
        unsafe { dst.set_len(0); }
    }

    let mut min = 100_000_000;
    let mut max = 0;
    for x in count.iter().skip(65).take(('Z' as u8 - 'A' as u8) as usize + 1) {
        min = if *x > 0 { std::cmp::min(min, *x) } else { min };
        max = std::cmp::max(max, *x);
    }
    let result = max - min;
    println!("result: {}", result);
}
