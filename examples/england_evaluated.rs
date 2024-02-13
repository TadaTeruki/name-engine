/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use placename_engine::{PlaceName, PlaceNameGeneratorBuilder, SyllableInfo};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn format(name: &String) -> String {
    let name = name.replace('+', " ").replace('*', "");
    name
}

fn normal_distribution(x: f64, mean: f64, std_dev: f64) -> f64 {
    let a = 1.0 / (std_dev * (2.0 * std::f64::consts::PI).sqrt());
    let b = -((x - mean).powi(2) / (2.0 * std_dev.powi(2)));
    a * b.exp()
}

fn evaluate(
    place_names: &Vec<PlaceName>,
    name: &String,
    _: &String,
    syllable_info: &Vec<SyllableInfo>,
) -> Option<f64> {
    // check if the same name is in the place_names
    let first_syllable_place_name = place_names[syllable_info[0].place_name_index].content();
    if first_syllable_place_name == *name {
        return None;
    }
    // check if the dulicate syllables are in the name, like 'Woodwood'
    let mut has_duplicate_syllables = false;
    syllable_info.iter().enumerate().for_each(|(i, _)| {
        if i == 0 {
            return;
        }
        let place_name_current = &place_names[syllable_info[i].place_name_index].syllables()
            [syllable_info[i].syllable_index];
        let place_name_previous = &place_names[syllable_info[i - 1].place_name_index].syllables()
            [syllable_info[i - 1].syllable_index];

        if place_name_current.0.to_uppercase() == place_name_previous.0.to_uppercase() {
            has_duplicate_syllables = true;
        }
    });
    if has_duplicate_syllables {
        return None;
    }

    // split syllables by '+'
    let syllables_splitted = syllable_info
        .iter()
        .fold(vec![vec![]], |mut acc, syllable_info| {
            let place_name = &place_names[syllable_info.place_name_index];
            let syllable = &place_name.syllables()[syllable_info.syllable_index];
            if syllable.0 == "+" {
                acc.push(vec![]);
            } else {
                acc.last_mut().unwrap().push(syllable_info);
            }
            acc
        });

    let max_syllable_count = syllables_splitted.iter().map(|s| s.len()).max().unwrap();
    let score_for_syllable_count = normal_distribution(max_syllable_count as f64, 2.25, 0.8);

    let block_count = syllables_splitted.len();
    let score_for_block_count = normal_distribution(block_count as f64, 1.5, 1.0);

    Some(score_for_syllable_count * score_for_block_count)
}

fn main() {
    let csv_file = include_str!("assets/england.csv");

    let place_names = csv_file
        .lines()
        .filter_map(|line| {
            let split = line.split(',');
            let phrases = split
                .last()
                .unwrap()
                .split(':')
                .map(|phrase| {
                    let mut split = phrase.split('_');
                    let name = split.next().unwrap();
                    let pronunciation = split.next().unwrap();
                    (name, pronunciation)
                })
                .collect::<Vec<(&str, &str)>>();
            if let Ok(placename) = PlaceName::new(phrases) {
                Some(placename)
            } else {
                None
            }
        })
        .collect::<Vec<PlaceName>>();

    let generator = PlaceNameGeneratorBuilder::new()
        .bulk_add_place_names(place_names)
        .build();

    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    (0..100).for_each(|_| {
        let evaluated = (0..3)
            .filter_map(|_| {
                let (name, pronunciation, syllable_info) = generator.generate_verbose(|| rng.gen());
                let score = evaluate(
                    generator.place_names(),
                    &name,
                    &pronunciation,
                    &syllable_info,
                );
                if let Some(score) = score {
                    Some((name, pronunciation, score))
                } else {
                    None
                }
            })
            .max_by(|(_, _, score1), (_, _, score2)| score1.partial_cmp(score2).unwrap());

        if let Some((name, pronunciation, _)) = evaluated {
            println!("{} /{}/", format(&name), format(&pronunciation));
        }
    });
}
