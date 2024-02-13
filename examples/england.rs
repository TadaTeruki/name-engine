/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use placename_engine::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn format(name: String) -> String {
    let name = name.replace('+', " ").replace('*', "");
    name
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
        let name = generator.generate(|| rng.gen());
        println!("{} /{}/", format(name.0), format(name.1));
    });
}
