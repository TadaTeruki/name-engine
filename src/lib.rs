use std::collections::HashMap;

use sorted_vec::{SortedVec, SortedVecBuilder};

type Notation = String;
type Pronunciation = String;
type ToRestore = bool;

mod sorted_vec;

struct PhonicsGraphBuilder {
    graph: HashMap<char, HashMap<char, usize>>,
}

impl PhonicsGraphBuilder {
    fn new() -> PhonicsGraphBuilder {
        PhonicsGraphBuilder {
            graph: HashMap::new(),
        }
    }

    fn add_char_pair(&mut self, incoming_char: char, outgoing_char: char) {
        self.graph
            .entry(incoming_char)
            .or_default()
            .entry(outgoing_char)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    fn build(self) -> PhonicsGraph {
        let mut builder = SortedVecBuilder::new();
        self.graph.iter().for_each(|(k, v)| {
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

        PhonicsGraph {
            graph: builder.build(),
        }
    }
}

struct PhonicsGraph {
    graph: SortedVec<char, SortedVec<f64, char>>,
}

impl PhonicsGraph {
    fn extract_forward(&self, character: char, prop: f64) -> char {
        let found = {
            let set = &self.graph.find(character).1;
            let found = set.find(prop);
            found
        };
        found.1
    }
}

#[derive(Debug)]
pub struct PlaceName {
    pub phrases: Vec<(Notation, Pronunciation)>,
}

impl PlaceName {
    pub fn new(phrases: Vec<(&str, &str)>) -> Self {
        Self {
            phrases: phrases
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
        let mut graph_builder = PhonicsGraphBuilder::new();
        let mut outgoing_tree = HashMap::new();
        let mut incoming_phrases = vec![];
        let mut outgoing_phrases = vec![];
        self.place_names.iter().for_each(|place_name| {
            for i in 0..place_name.phrases.len() - 1 {
                let (incoming_char, outgoing_char) = (
                    place_name.phrases[i].1.chars().last(),
                    place_name.phrases[i + 1].1.chars().next(),
                );
                if incoming_char.is_none() || outgoing_char.is_none() {
                    continue;
                }
                let (incoming_char, outgoing_char) =
                    (incoming_char.unwrap(), outgoing_char.unwrap());
                graph_builder.add_char_pair(incoming_char, outgoing_char);
                if i == 0 {
                    incoming_phrases.push(place_name.phrases[i].clone());
                }
                let to_restore = i + 1 != place_name.phrases.len() - 1;
                outgoing_phrases.push((
                    place_name.phrases[i + 1].0.clone(),
                    place_name.phrases[i + 1].1.clone(),
                    to_restore,
                ));
                outgoing_tree
                    .entry(outgoing_char)
                    .and_modify(|v: &mut Vec<usize>| v.push(outgoing_phrases.len() - 1))
                    .or_insert(vec![outgoing_phrases.len() - 1]);
            }
        });

        PlaceNameGenerator {
            incoming_phrases,
            outgoing_phrases,
            outgoing_tree,
            graph: graph_builder.build(),
        }
    }
}

pub struct PlaceNameGenerator {
    // PlaceNameのincomingを格納する木構造
    incoming_phrases: Vec<(Notation, Pronunciation)>,
    // PlaceNameのoutgoingを格納する木構造
    outgoing_phrases: Vec<(Notation, Pronunciation, ToRestore)>,
    // 各Phonicsから始まるoutgoing phraseのindexを格納する木構造
    outgoing_tree: HashMap<char, Vec<usize>>,
    // Phonics同士の接続を表すグラフ
    graph: PhonicsGraph,
}

impl PlaceNameGenerator {
    pub fn generate(&self, mut rand_fn: impl FnMut() -> f64) -> (Notation, Pronunciation) {
        let query_next = |incoming_phrase: (Notation, Pronunciation), p0: f64, p1: f64| {
            let connection_phrase = self
                .graph
                .extract_forward(incoming_phrase.1.chars().last().unwrap(), p0);
            let outgoing_phrase_list = &self.outgoing_tree.get(&connection_phrase).unwrap();
            let outgoing_phrase = &self.outgoing_phrases
                [outgoing_phrase_list[(p1 * outgoing_phrase_list.len() as f64) as usize]];
            (
                outgoing_phrase.0.clone(),
                outgoing_phrase.1.clone(),
                outgoing_phrase.2,
            )
        };

        let incoming_phrase =
            &self.incoming_phrases[(rand_fn() * self.incoming_phrases.len() as f64) as usize];
        let mut phrases_vec = vec![(incoming_phrase.0.clone(), incoming_phrase.1.clone())];

        let mut restore_flag = true;
        while restore_flag {
            let (k, r, to_restore) = query_next(
                phrases_vec[phrases_vec.len() - 1].clone(),
                rand_fn(),
                rand_fn(),
            );
            phrases_vec.push((k, r));
            restore_flag = to_restore;
        }

        // 圧縮
        let phrases_vec = phrases_vec
            .iter()
            .enumerate()
            .filter(|(i, _)| {
                if *i == 0 {
                    return true;
                }
                phrases_vec[*i - 1].0 != phrases_vec[*i].0
                    || phrases_vec[*i - 1].1.chars().last().unwrap()
                        != phrases_vec[*i].1.chars().last().unwrap()
            })
            .map(|(_, p)| p.clone())
            .collect::<Vec<(Notation, Pronunciation)>>();

        let notation = phrases_vec
            .iter()
            .map(|p| p.0.clone())
            .collect::<Vec<Notation>>()
            .join("");
        let pronunciation = phrases_vec
            .iter()
            .map(|p| p.1.clone())
            .collect::<Vec<Pronunciation>>()
            .join("");

        (notation, pronunciation)
    }
}
