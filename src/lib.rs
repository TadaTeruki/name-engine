type Kanji = String;
type Romaji = String;

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
    fn extract_forward(&self, ascii: Ascii, prop: f64) -> Ascii {
        let code = ascii.to_code() as usize;
        let outgoing_items = &self.graph[code].0;
        let outgoing_sum = self.graph[code].1;
        let to_extract = (outgoing_sum as f64 * prop) as usize;
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
    pub incoming_phrase: (Kanji, Romaji),
    pub outgoing_phrase: (Kanji, Romaji),
}

impl PlaceName {
    pub fn new(
        incoming_kanji: &str,
        incoming_romaji: &str,
        outgoing_kanji: &str,
        outgoing_romaji: &str,
    ) -> Self {
        Self {
            incoming_phrase: (incoming_kanji.to_string(), incoming_romaji.to_string()),
            outgoing_phrase: (outgoing_kanji.to_string(), outgoing_romaji.to_string()),
        }
    }
}

pub struct PlaceNameGeneratorBuilder {
    place_names: Vec<PlaceName>,
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
        self.place_names.iter().enumerate().for_each(|(i, place_name)| {
            if let (Some(incoming_char), Some(outgoing_char)) = (place_name.incoming_phrase.1.chars().last(), place_name.outgoing_phrase.1.chars().next()) {
                let incoming_ascii = Ascii::from_char(incoming_char);
                let outgoing_ascii = Ascii::from_char(outgoing_char);
                graph_builder.add_ascii_pair(incoming_ascii, outgoing_ascii);
                outgoing_tree[outgoing_ascii.to_code() as usize].1.push(i);
                incoming_phrases.push(place_name.incoming_phrase.clone());
                outgoing_phrases.push(place_name.outgoing_phrase.clone());
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
    outgoing_phrases: Vec<(Kanji, Romaji)>,
    // 各Phonicsから始まるoutgoing phraseのindexを格納する木構造
    outgoing_tree: Vec<(Ascii, Vec<usize>)>,
    // Phonics同士の接続を表すグラフ
    graph: PhonicsGraph,
}

impl PlaceNameGenerator {

    pub fn generate(&self, p0: f64, p1: f64, p2: f64) -> (Kanji, Romaji) {
        let incoming_phrase_code = (self.incoming_phrases.len() as f64 * p0) as usize;
        let incoming_phrase = self.incoming_phrases[incoming_phrase_code].clone();
        let connection_phrase = self.graph.extract_forward(
            Ascii::from_char(incoming_phrase.1.chars().last().unwrap()),
            p1,
        );
        let outgoing_phrase_code = (self.outgoing_tree[connection_phrase.to_code() as usize]
            .1
            .len() as f64
            * p2) as usize;
        let outgoing_phrase = self.outgoing_phrases
            [self.outgoing_tree[connection_phrase.to_code() as usize].1[outgoing_phrase_code]]
            .clone();
        (
            incoming_phrase.0 + &outgoing_phrase.0,
            incoming_phrase.1 + &outgoing_phrase.1,
        )
    }
}
