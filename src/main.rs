use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::Write;

fn modulo_positive(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

fn get_nth_bit(value: u8, n: u8) -> bool {
    ((value >> n) & 1) == 1
}

fn tile_string(tile: &u8) -> String {
    let ts: Vec<char> = (0..8)
        .map(|i| if get_nth_bit(*tile, i) { '■' } else { '□' })
        .collect();
    format!(
        "{} {} {}\n{} □ {}\n{} {} {}",
        ts[0], ts[1], ts[2], ts[7], ts[3], ts[6], ts[5], ts[4]
    )
}

fn is_tile_valid(tile: &u8) -> bool {
    for n in (0..8).step_by(2) {
        if !get_nth_bit(*tile, n) {
            if get_nth_bit(*tile, modulo_positive(n as i32 - 1, 8) as u8) {
                return false;
            }
            if get_nth_bit(*tile, modulo_positive(n as i32 + 1, 8) as u8) {
                return false;
            }
        }
    }
    true
}

fn join_tile_string(s0: String, s1: String) -> String {
    s0.lines()
        .zip(s1.lines())
        .map(|(l0, l1)| format!("{}\t{}", l0, l1))
        .reduce(|p, q| format!("{}\n{}", p, q))
        .unwrap()
}

fn bits_pattern_match(p: u8, q: u8) -> bool {
    if p == q {
        return true;
    }
    for n in (2..8).step_by(2) {
        if ((p << n) | (p >> (8 - n))) & 0xFF == q {
            return true;
        }
    }
    return false;
}

fn organise_tiles_by_pattern(tiles: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut organised_tiles: HashMap<u8, Vec<u8>> = HashMap::new();
    for tile in tiles {
        let mut tile_added = false;
        for (key, tile_group) in &mut organised_tiles {
            if bits_pattern_match(*tile, *key) {
                tile_group.push(*tile);
                tile_added = true;
                break;
            }
        }
        if !tile_added {
            organised_tiles.insert(*tile, vec![*tile]);
        }
    }
    organised_tiles.values().cloned().collect()
}

fn organise_tiles_by_num_ones(tiles: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut organised_tiles = Vec::new();
    for tile in tiles {
        let num_ones = tile.count_ones() as usize;
        for _ in organised_tiles.len()..num_ones + 1 {
            organised_tiles.push(Vec::new());
        }
        organised_tiles[num_ones].push(*tile);
    }
    organised_tiles
}

fn main() {
    // get all valid tiles
    let tiles: Vec<u8> = (u8::MIN..=u8::MAX).filter(is_tile_valid).collect();

    // organise onto groups of all the same number of bits
    let organised_tiles = organise_tiles_by_pattern(&tiles);

    // print out the tiles
    let tiles_string = organised_tiles
        .iter()
        .map(|tile_group| {
            tile_group
                .iter()
                .map(tile_string)
                .reduce(|s0, s1| join_tile_string(s0, s1))
                .unwrap()
        })
        .reduce(|tile_group_string_0, tile_group_string_1| {
            format!("{}\n\n{}", tile_group_string_0, tile_group_string_1)
        })
        .unwrap();

    let mut f = File::create("output.txt").unwrap();
    write!(f, "{}", tiles_string).unwrap();
}