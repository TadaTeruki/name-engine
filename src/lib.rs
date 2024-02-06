/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use sorted_vec::{SortedVec, SortedVecBuilder};

type Notation = String;
type Pronunciation = String;
type ToRestore = bool;

mod sorted_vec;

struct PhoneticConnectionBuilder {
    conn: HashMap<char, HashMap<char, usize>>,
}

impl PhoneticConnectionBuilder {
    fn new() -> PhoneticConnectionBuilder {
        PhoneticConnectionBuilder {
            conn: HashMap::new(),
        }
    }

    fn add_char_pair(&mut self, incoming_char: char, outgoing_char: char) {
        self.conn
            .entry(incoming_char)
            .or_default()
            .entry(outgoing_char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn build(self) -> PhoneticConnection {
        let mut builder = SortedVecBuilder::new();
        self.conn.iter().for_each(|(k, v)| {
            let mut sum = 0;
            for v2 in v.values() {
                sum += v2;
            }

            let mut v = v.iter().collect::<Vec<(&char, &usize)>>();
            v.sort_by(|a, b| a.0.cmp(b.0));

            let mut set = SortedVecBuilder::new();
            let mut prop = 0.0;

            for (k2, v2) in v {
                prop += *v2 as f64 / sum as f64;
                set.push(prop, *k2);
            }

            let set = set.build();
            builder.push(*k, set);
        });

        PhoneticConnection {
            conn: builder.build(),
        }
    }
}

struct PhoneticConnection {
    conn: SortedVec<char, SortedVec<f64, char>>,
}

impl PhoneticConnection {
    fn extract_forward(&self, character: char, prop: f64) -> char {
        let found = {
            let set = &self.conn.find(character).1;
            let found = set.find(prop);
            found
        };
        found.1
    }
}

#[derive(Debug)]
pub struct PlaceName {
    pub blocks: Vec<(Notation, Pronunciation)>,
}

impl PlaceName {
    pub fn new(blocks: Vec<(&str, &str)>) -> Self {
        Self {
            blocks: blocks
                .iter()
                .map(|(k, r)| (k.to_string(), r.to_string()))
                .collect(),
        }
    }
}

pub struct PlaceNameGeneratorBuilder {
    place_names: Vec<PlaceName>,
}

impl Default for PlaceNameGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PlaceNameGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            place_names: vec![],
        }
    }

    pub fn add_place_name(mut self, place_name: PlaceName) -> Self {
        self.place_names.push(place_name);
        self
    }

    pub fn bulk_add_place_names(mut self, place_names: Vec<PlaceName>) -> Self {
        self.place_names.extend(place_names);
        self
    }

    pub fn build(self) -> PlaceNameGenerator {
        let mut conn_builder = PhoneticConnectionBuilder::new();
        let mut outgoing_tree = HashMap::new();
        let mut incoming_blocks = vec![];
        let mut outgoing_blocks = vec![];
        self.place_names.iter().for_each(|place_name| {
            for i in 0..place_name.blocks.len() - 1 {
                let (incoming_char, outgoing_char) = (
                    place_name.blocks[i].1.chars().last(),
                    place_name.blocks[i + 1].1.chars().next(),
                );
                if incoming_char.is_none() || outgoing_char.is_none() {
                    continue;
                }
                let (incoming_char, outgoing_char) =
                    (incoming_char.unwrap(), outgoing_char.unwrap());
                conn_builder.add_char_pair(incoming_char, outgoing_char);
                if i == 0 {
                    incoming_blocks.push(place_name.blocks[i].clone());
                }
                let to_restore = i + 1 != place_name.blocks.len() - 1;
                outgoing_blocks.push((
                    place_name.blocks[i + 1].0.clone(),
                    place_name.blocks[i + 1].1.clone(),
                    to_restore,
                ));
                outgoing_tree
                    .entry(outgoing_char)
                    .and_modify(|v: &mut Vec<usize>| v.push(outgoing_blocks.len() - 1))
                    .or_insert(vec![outgoing_blocks.len() - 1]);
            }
        });

        PlaceNameGenerator {
            incoming_blocks,
            outgoing_blocks,
            outgoing_tree,
            conn: conn_builder.build(),
        }
    }
}

pub struct PlaceNameGenerator {
    // blocks that can be the first block
    incoming_blocks: Vec<(Notation, Pronunciation)>,
    // blocks that can be the next block
    outgoing_blocks: Vec<(Notation, Pronunciation, ToRestore)>,
    // list of the index of the outgoing_blocks which has the same first character
    outgoing_tree: HashMap<char, Vec<usize>>,
    // phonetic connection between the last character of the previous block and the first character of the next block
    conn: PhoneticConnection,
}

impl PlaceNameGenerator {
    pub fn generate(&self, mut rand_fn: impl FnMut() -> f64) -> (Notation, Pronunciation) {
        let query_next = |incoming_block: (Notation, Pronunciation), p0: f64, p1: f64| {
            let connection_block = self
                .conn
                .extract_forward(incoming_block.1.chars().last().unwrap(), p0);
            let outgoing_block_list = &self.outgoing_tree.get(&connection_block).unwrap();
            let outgoing_block = &self.outgoing_blocks
                [outgoing_block_list[(p1 * outgoing_block_list.len() as f64) as usize]];
            (
                outgoing_block.0.clone(),
                outgoing_block.1.clone(),
                outgoing_block.2,
            )
        };

        let incoming_block =
            &self.incoming_blocks[(rand_fn() * self.incoming_blocks.len() as f64) as usize];
        let mut blocks_vec = vec![(incoming_block.0.clone(), incoming_block.1.clone())];

        let mut restore_flag = true;
        while restore_flag {
            let (k, r, to_restore) = query_next(
                blocks_vec[blocks_vec.len() - 1].clone(),
                rand_fn(),
                rand_fn(),
            );
            blocks_vec.push((k, r));
            restore_flag = to_restore;
        }

        // If the last block is the same as the previous one, remove it
        let blocks_vec = blocks_vec
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                if *i == 0 {
                    return true;
                }
                blocks_vec[*i - 1].0 != blocks_vec[*i].0
                    || blocks_vec[*i - 1].1.chars().last().unwrap()
                        != blocks_vec[*i].1.chars().last().unwrap()
            })
            .map(|(_, p)| p.clone())
            .collect::<Vec<(Notation, Pronunciation)>>();

        let notation = blocks_vec
            .iter()
            .map(|p| p.0.clone())
            .collect::<Vec<Notation>>()
            .join("");
        let pronunciation = blocks_vec
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<Pronunciation>>()
            .join("");

        (notation, pronunciation)
    }
}
