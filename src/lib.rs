type Kanji = String;
type Romaji = String;
type ToRestore = bool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ascii {
    code: u8,
}

#[allow(dead_code)]
impl Ascii {
    pub fn from_code(code: u8) -> Ascii {
        Ascii { code }
    }

    pub fn from_char(c: char) -> Ascii {
        Ascii {
            code: (c as u8) - 97,
        }
    }

    pub fn to_char(self) -> char {
        (self.code + 97) as char
    }

    pub fn to_code(self) -> u8 {
        self.code
    }
}

struct PhonicsGraphBuilder {
    // [([outgoing_ascii, count], count_sum)]
    graph: Vec<(Vec<(Ascii, usize)>, usize)>,
}

impl PhonicsGraphBuilder {
    fn new() -> PhonicsGraphBuilder {
        PhonicsGraphBuilder {
            graph: vec![(vec![], 0); 26],
        }
    }

    fn add_ascii_pair(&mut self, incoming_ascii: Ascii, outgoing_ascii: Ascii) {
        let incoming_code = incoming_ascii.to_code() as usize;
        let mut found = false;
        for i in 0..self.graph[incoming_code].0.len() {
            if self.graph[incoming_code].0[i].0 == outgoing_ascii {
                self.graph[incoming_code].0[i].1 += 1;
                found = true;
                break;
            }
        }
        if !found {
            self.graph[incoming_code].0.push((outgoing_ascii, 1));
        }
        self.graph[incoming_code].1 += 1;
    }

    fn build(self) -> PhonicsGraph {
        PhonicsGraph { graph: self.graph }
    }
}

struct PhonicsGraph {
    // [([outgoing_ascii, count], count_sum)]
    graph: Vec<(Vec<(Ascii, usize)>, usize)>,
}

impl PhonicsGraph {
    fn extract_forward(&self, ascii: Ascii, index: usize) -> Ascii {
        let code = ascii.to_code() as usize;
        let outgoing_items = &self.graph[code].0;
        let outgoing_sum = self.graph[code].1;
        let to_extract = index % outgoing_sum;
        let mut sum = 0;
        for item in outgoing_items {
            sum += item.1;
            if sum >= to_extract {
                return item.0;
            }
        }
        outgoing_items[outgoing_items.len() - 1].0
    }
}

pub struct PlaceName {
    pub phrases: Vec<(Kanji, Romaji)>,
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
        let mut outgoing_tree = (0..26)
            .map(|i| (Ascii::from_code(i), vec![]))
            .collect::<Vec<(Ascii, Vec<usize>)>>();
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
                let (incoming_ascii, outgoing_ascii) = (
                    Ascii::from_char(incoming_char),
                    Ascii::from_char(outgoing_char),
                );
                graph_builder.add_ascii_pair(incoming_ascii, outgoing_ascii);
                if i == 0 {
                    incoming_phrases.push(place_name.phrases[i].clone());
                }
                let to_restore = i + 1 != place_name.phrases.len() - 1;
                outgoing_phrases.push((
                    place_name.phrases[i + 1].0.clone(),
                    place_name.phrases[i + 1].1.clone(),
                    to_restore,
                ));
                outgoing_tree[outgoing_ascii.to_code() as usize]
                    .1
                    .push(outgoing_phrases.len() - 1);
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
    incoming_phrases: Vec<(Kanji, Romaji)>,
    // PlaceNameのoutgoingを格納する木構造
    outgoing_phrases: Vec<(Kanji, Romaji, ToRestore)>,
    // 各Phonicsから始まるoutgoing phraseのindexを格納する木構造
    outgoing_tree: Vec<(Ascii, Vec<usize>)>,
    // Phonics同士の接続を表すグラフ
    graph: PhonicsGraph,
}

impl PlaceNameGenerator {
    pub fn generate(&self, mut pfunc: impl FnMut() -> usize) -> (Kanji, Romaji) {
        let append = |incoming_phrase: (Kanji, Romaji), p0: usize, p1: usize| {
            let connection_phrase = self.graph.extract_forward(
                Ascii::from_char(incoming_phrase.1.chars().last().unwrap()),
                p0,
            );

            let outgoing_phrase_list = &self.outgoing_tree[connection_phrase.to_code() as usize];
            let outgoing_phrase_code = p1 % outgoing_phrase_list.1.len();
            let outgoing_phrase =
                self.outgoing_phrases[outgoing_phrase_list.1[outgoing_phrase_code]].clone();
            (
                incoming_phrase.0 + &outgoing_phrase.0,
                incoming_phrase.1 + &outgoing_phrase.1,
                outgoing_phrase.2,
            )
        };

        let incoming_phrase_code = pfunc() % self.incoming_phrases.len();
        let incoming_phrase = self.incoming_phrases[incoming_phrase_code].clone();

        let mut incoming_phrase = append(incoming_phrase, pfunc(), pfunc());
        while incoming_phrase.2 {
            incoming_phrase = append((incoming_phrase.0, incoming_phrase.1), pfunc(), pfunc());
        }

        (incoming_phrase.0, incoming_phrase.1)
    }
}
