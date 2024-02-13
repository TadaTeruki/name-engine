/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use thiserror::Error;

use sorted_vec::{SortedVec, SortedVecBuilder};

/// The letter of the syllable
type Letter = String;
/// The phonetic representation of the letter
type Phonics = String;
/// The parameter of syllables that can be the next syllable or not
type ToRestore = bool;

mod sorted_vec;

#[derive(Error, Debug)]
pub enum NameError {
    #[error("empty string detected")]
    EmptyString,
}

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

/// The struct that represents the name.
///  names are composed of syllables, and each syllable has a letter as `Letter`, and a phonetic representation as `Phonics`.
///
/// Example: Bedford -> Name::new(vec![("bed", "ˈbɛd"), ("ford", "fərd")])
#[derive(Debug)]
pub struct Name {
    syllables: Vec<(Letter, Phonics)>,
}

impl Name {
    pub fn new(syllables: Vec<(&str, &str)>) -> Result<Self, NameError> {
        Self::from_string(
            syllables
                .iter()
                .map(|(k, r)| (k.to_string(), r.to_string()))
                .collect(),
        )
    }

    pub fn from_string(syllables: Vec<(String, String)>) -> Result<Self, NameError> {
        for syllable in &syllables {
            if syllable.1.is_empty() {
                return Err(NameError::EmptyString);
            }
        }
        Ok(Self { syllables })
    }

    fn connection_pairs(&self) -> Vec<(char, char)> {
        let mut pairs = vec![];
        for i in 0..self.syllables.len() - 1 {
            pairs.push((
                self.syllables[i].1.chars().last().unwrap(),
                self.syllables[i + 1].1.chars().next().unwrap(),
            ))
        }
        pairs
    }

    fn last_char_of_syllable(&self, i: usize) -> char {
        self.syllables[i].1.chars().last().unwrap()
    }

    pub fn content(&self) -> Letter {
        self.syllables.iter().map(|p| p.0.clone()).collect()
    }

    pub fn script(&self) -> Phonics {
        self.syllables.iter().map(|p| p.1.clone()).collect()
    }

    pub fn syllables(&self) -> &Vec<(Letter, Phonics)> {
        &self.syllables
    }
}

/// The builder for the NameGenerator.
pub struct NameGeneratorBuilder {
    names: Vec<Name>,
}

impl Default for NameGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl NameGeneratorBuilder {
    pub fn new() -> Self {
        Self { names: vec![] }
    }

    pub fn add_name(mut self, name: Name) -> Self {
        self.names.push(name);
        self
    }

    pub fn bulk_add_names(mut self, names: Vec<Name>) -> Self {
        self.names.extend(names);
        self
    }

    pub fn build(self) -> NameGenerator {
        let mut conn_builder = PhoneticConnectionBuilder::new();
        let mut outgoing_tree = HashMap::new();
        let mut incoming_syllables = vec![];
        let mut outgoing_syllables = vec![];
        self.names.iter().enumerate().for_each(|(ipn, name)| {
            name.connection_pairs()
                .iter()
                .enumerate()
                .for_each(|(ipc, pair)| {
                    conn_builder.add_char_pair(pair.0, pair.1);
                    if ipc == 0 {
                        incoming_syllables.push((ipn, ipc));
                    }
                    let to_restore = ipc + 1 != name.syllables.len() - 1;
                    outgoing_syllables.push((ipn, ipc + 1, to_restore));
                    outgoing_tree
                        .entry(pair.1)
                        .and_modify(|v: &mut Vec<usize>| v.push(outgoing_syllables.len() - 1))
                        .or_insert(vec![outgoing_syllables.len() - 1]);
                });
        });

        NameGenerator {
            names: self.names,
            incoming_syllables,
            outgoing_syllables,
            outgoing_tree,
            conn: conn_builder.build(),
        }
    }
}

/// The generator for the names.
pub struct NameGenerator {
    // list of the names
    names: Vec<Name>,
    // syllables that can be the first syllable
    incoming_syllables: Vec<(usize, usize)>,
    // syllables that can be the next syllable
    outgoing_syllables: Vec<(usize, usize, ToRestore)>,
    // list of the index of the outgoing_syllables which has the same first character
    outgoing_tree: HashMap<char, Vec<usize>>,
    // phonetic connection between the last character of the previous syllable and the first character of the next syllable
    conn: PhoneticConnection,
}

/// The detailed information of the syllables.
#[derive(Debug)]
pub struct SyllableInfo {
    /// The index of the name in the dataset
    pub name_index: usize,
    /// The index of the syllable in the name
    pub syllable_index: usize,
}

impl NameGenerator {
    /// Generate a name with detailed information of the syllables.
    /// Random number generator is required as argument `rand_fn`.
    pub fn generate_verbose(
        &self,
        mut rand_fn: impl FnMut() -> f64,
    ) -> (Letter, Phonics, Vec<SyllableInfo>) {
        let query_next = |incoming_syllable: (usize, usize), p0: f64, p1: f64| {
            let connection_syllable = self.conn.extract_forward(
                self.names[incoming_syllable.0].last_char_of_syllable(incoming_syllable.1),
                p0,
            );
            let outgoing_syllable_list = &self.outgoing_tree[&connection_syllable];
            &self.outgoing_syllables
                [outgoing_syllable_list[(p1 * outgoing_syllable_list.len() as f64) as usize]]
        };

        let incoming_syllable =
            &self.incoming_syllables[(rand_fn() * self.incoming_syllables.len() as f64) as usize];
        let mut syllables_vec = vec![(incoming_syllable.0, incoming_syllable.1)];

        let mut restore_flag = true;
        while restore_flag {
            let (k, r, to_restore) =
                query_next(syllables_vec[syllables_vec.len() - 1], rand_fn(), rand_fn());
            syllables_vec.push((*k, *r));
            restore_flag = *to_restore;
        }

        let syllable_info = syllables_vec
            .iter()
            .map(|p| SyllableInfo {
                name_index: p.0,
                syllable_index: p.1,
            })
            .collect::<Vec<SyllableInfo>>();

        let content = syllables_vec
            .iter()
            .map(|p| self.names[p.0].syllables[p.1].0.clone())
            .collect::<Vec<Letter>>()
            .join("");
        let script = syllables_vec
            .iter()
            .map(|p| self.names[p.0].syllables[p.1].1.clone())
            .collect::<Vec<Phonics>>()
            .join("");

        (content, script, syllable_info)
    }

    /// Generate a name.
    /// Random number generator is required as argument `rand_fn`.
    pub fn generate(&self, rand_fn: impl FnMut() -> f64) -> (Letter, Phonics) {
        let (content, script, _) = self.generate_verbose(rand_fn);
        (content, script)
    }

    /// Get the list of the names as reference
    pub fn names(&self) -> &Vec<Name> {
        &self.names
    }
}
