/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use placename_engine::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let csv_file = include_str!("assets/hokkaido.csv");

    let place_names = csv_file
        .lines()
        .map(|line| {
            let split = line.split(',');
            let phrases = split
                .last()
                .unwrap()
                .split(':')
                .map(|phrase| {
                    let mut split = phrase.split('_');
                    (split.next().unwrap(), split.next().unwrap())
                })
                .collect::<Vec<(&str, &str)>>();
            PlaceName::new(phrases)
        })
        .collect::<Vec<PlaceName>>();

    let generator = PlaceNameGeneratorBuilder::new()
        .bulk_add_place_names(place_names)
        .build();
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    (0..100).for_each(|_| {
        let name = generator.generate(|| rng.gen());
        println!("{} {}", name.0, name.1);
    });
}
