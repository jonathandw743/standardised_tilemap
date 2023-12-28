#![allow(dead_code, unused_variables)]

use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

fn modulo_positive(n: i32, m: i32) -> i32 {
    ((n % m) + m) % m
}

fn get_nth_bit(value: u8, n: u8) -> bool {
    ((value >> n) & 1) == 1
}

fn circular_left_shift(value: u8, shift: usize) -> u8 {
    if shift % 8 == 0 {
        return value;
    }
    ((value << shift) | (value >> (8 - shift))) & 0xFF
}

fn circular_right_shift(value: u8, shift: usize) -> u8 {
    if shift % 8 == 0 {
        return value;
    }
    ((value >> shift) | (value << (8 - shift))) & 0xFF
}

fn tile_string(tile: u8) -> String {
    let ts: Vec<char> = (0..8)
        .map(|i| if get_nth_bit(tile, i) { '■' } else { '□' })
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

fn join_tile_string(s0: String, s1: String, spacing: &str) -> String {
    s0.lines()
        .zip(s1.lines())
        .map(|(l0, l1)| format!("{}{}{}", l0, spacing, l1))
        .reduce(|p, q| format!("{}\n{}", p, q))
        .unwrap()
}

fn reverse_bits(n: u8) -> u8 {
    let mut temp = n;
    let mut result = 0;

    for _ in 0..8 {
        result = (result << 1) | (temp & 1);
        temp >>= 1;
    }

    result
}

fn bits_circular_pattern_match(p: u8, q: u8) -> bool {
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

fn bits_circular_pattern_match_reverse(p: u8, q: u8) -> bool {
    let reverse_q = reverse_bits(q);
    if p == reverse_q {
        return true;
    }
    for n in (1..8).step_by(2) {
        if ((p << n) | (p >> (8 - n))) & 0xFF == reverse_q {
            return true;
        }
    }
    return false;
}

fn organise_tiles_by_pattern_opposites_allowed(tiles: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut organised_tiles: HashMap<u8, Vec<u8>> = HashMap::new();
    for tile in tiles {
        let mut tile_added = false;
        for (key, tile_group) in &mut organised_tiles {
            if bits_circular_pattern_match(*tile, *key)
                || bits_circular_pattern_match_reverse(*tile, *key)
            {
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

fn organise_tiles_by_pattern_opposites_disallowed(tiles: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut organised_tiles: HashMap<u8, Vec<u8>> = HashMap::new();
    for tile in tiles {
        let mut tile_added = false;
        for (key, tile_group) in &mut organised_tiles {
            if bits_circular_pattern_match(*tile, *key) {
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

// use enumflags2::bitflags;

// #[repr(u8)]  // default: #[repr(usize)]
// #[bitflags]
// enum Adjacency {
//     Top = 0b01000000,
//     Right = 0b00010000,
//     Bottom = 0b00000100,
//     Left = 0b00000001,

//     TopLeft = 0b11000001,
//     TopRight = 0b01110000,
//     BottomRight = 0b00011100,
//     BottomLeft = 0b00000111,
// }

// const ADJACENCIES: [u8; 8] = [
//     0b11000001, 0b01000000, 0b01110000, 0b00010000, 0b00011100, 0b00000100, 0b00000111, 0b00000001,
// ];

// fn create_adjacency_map(tile: u8) -> u8 {
//     let mut adjacency = 0u8;
//     for n in 0..8 {
//         if get_nth_bit(tile, n as u8) {
//             adjacency |= ADJACENCIES[n];
//         }
//     }
//     adjacency
// }

fn create_adjacency_map(tiles: &Vec<u8>) -> HashMap<u8, [HashSet<u8>; 8]> {
    let mut result = HashMap::new();
    for tile in tiles {
        let mut possible_adjacent_tiles: [HashSet<u8>; 8] = Default::default();
        for other_tile in tiles {
            
            for n in (1..8).step_by(2) {
                let mask = 0b10000011;
                if (circular_right_shift(*tile, n) & mask)
                    == circular_right_shift(reverse_bits(*other_tile), match n {
                        1 => 2,
                        3 => 0,
                        5 => 6,
                        7 => 4,
                        _ => 0
                    })
                        & mask
                {
                    // }
                    // if !get_nth_bit(*tile, n as u8) && !get_nth_bit(*other_tile, (n as u8 + 4) % 8) {
                    possible_adjacent_tiles[n].insert(*other_tile);
                }
            }
        }
        result.insert(*tile, possible_adjacent_tiles);
    }
    result
}

fn random_tile(tiles: &HashSet<u8>, rng: &mut ThreadRng) -> Option<u8> {
    let tiles_vec: Vec<u8> = tiles.into_iter().map(|tile| *tile).collect();
    tiles_vec.choose(rng).map(|tile| *tile)
}

fn optional_tile_string(tile: Option<u8>) -> String {
    match tile {
        Some(tile) => tile_string(tile),
        None => "     \n     \n     ".into(),
    }
}

fn main() {
    // assert_eq!(circular_left_shift(0b00000111, 0), 0b00000111);
    // assert_eq!(circular_left_shift(0b00000111, 1), 0b00001110);
    // assert_eq!(circular_left_shift(0b00000111, 2), 0b00011100);
    // assert_eq!(circular_left_shift(0b00000111, 3), 0b00111000);
    // assert_eq!(circular_left_shift(0b00000111, 4), 0b01110000);
    // assert_eq!(circular_left_shift(0b00000111, 5), 0b11100000);
    // assert_eq!(circular_left_shift(0b00000111, 6), 0b11000001);
    // assert_eq!(circular_left_shift(0b00000111, 7), 0b10000011);
    // assert_eq!(circular_left_shift(0b00000111, 8), 0b00000111);
    // assert_eq!(circular_left_shift(0b00000111, 9), 0b00001110);
    // assert_eq!(circular_left_shift(0b00000111, 10), 0b00011100);

    // assert_eq!(circular_right_shift(0b00000111, 0), 0b00000111);
    // assert_eq!(circular_right_shift(0b00000111, 1), 0b10000011);
    // assert_eq!(circular_right_shift(0b00000111, 2), 0b11000001);
    // assert_eq!(circular_right_shift(0b00000111, 3), 0b11100000);
    // assert_eq!(circular_right_shift(0b00000111, 4), 0b01110000);
    // assert_eq!(circular_right_shift(0b00000111, 5), 0b00111000);
    // assert_eq!(circular_right_shift(0b00000111, 6), 0b00011100);
    // assert_eq!(circular_right_shift(0b00000111, 7), 0b00001110);
    // assert_eq!(circular_right_shift(0b00000111, 8), 0b00000111);
    // assert_eq!(circular_right_shift(0b00000111, 9), 0b10000011);
    // assert_eq!(circular_right_shift(0b00000111, 10), 0b11000001);
    // get all valid tiles
    let tiles = (u8::MIN..=u8::MAX).filter(is_tile_valid).collect();

    let adjacency_map = create_adjacency_map(&tiles);

    // organise onto groups of all the same number of bits
    let tiles_organised_by_pattern_opposites_allowed =
        organise_tiles_by_pattern_opposites_allowed(&tiles);
    let tiles_organised_by_pattern_opposites_disallowed =
        organise_tiles_by_pattern_opposites_disallowed(&tiles);
    let tiles_organised_by_num_ones = organise_tiles_by_num_ones(&tiles);

    // print out the tiles
    let tiles_string = tiles_organised_by_pattern_opposites_allowed
        .iter()
        .map(|tile_group| {
            tile_group
                .iter()
                .map(|tile| tile_string(*tile))
                .reduce(|s0, s1| join_tile_string(s0, s1, "   "))
                .unwrap()
        })
        .reduce(|tile_group_string_0, tile_group_string_1| {
            format!("{}\n\n{}", tile_group_string_0, tile_group_string_1)
        })
        .unwrap();

    let mut rng = rand::thread_rng();

    let tiles_adjacency_string = adjacency_map
        .iter()
        .map(|(tile, possible_adj_tiles)| {
            let header_footer = "+---------------------------------------------+";
            format!(
                "{}\n{}\n{}\n{}\n{}",
                header_footer,
                join_tile_string(
                    join_tile_string(
                        optional_tile_string(random_tile(&possible_adj_tiles[0], &mut rng)),
                        optional_tile_string(random_tile(&possible_adj_tiles[1], &mut rng)),
                        " "
                    ),
                    optional_tile_string(random_tile(&possible_adj_tiles[2], &mut rng)),
                    " "
                ),
                join_tile_string(
                    join_tile_string(
                        optional_tile_string(random_tile(&possible_adj_tiles[7], &mut rng)),
                        tile_string(*tile),
                        " "
                    ),
                    optional_tile_string(random_tile(&possible_adj_tiles[3], &mut rng)),
                    " "
                ),
                join_tile_string(
                    join_tile_string(
                        optional_tile_string(random_tile(&possible_adj_tiles[6], &mut rng)),
                        optional_tile_string(random_tile(&possible_adj_tiles[5], &mut rng)),
                        " "
                    ),
                    optional_tile_string(random_tile(&possible_adj_tiles[4], &mut rng)),
                    " "
                ),
                header_footer
            )
        })
        .reduce(|s0, s1| format!("{}\n\n{}", s0, s1))
        .unwrap();
    let mut f = File::create("tiles.txt").unwrap();
    write!(f, "{}", tiles_string).unwrap();
    let mut f2 = File::create("adjacent_tiles.txt").unwrap();
    write!(f2, "{}", tiles_adjacency_string).unwrap();
}
