use name_engine::{Name, NameGeneratorBuilder};
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
            if let Ok(name) = Name::new(phrases) {
                Some(name)
            } else {
                None
            }
        })
        .collect::<Vec<Name>>();

    let generator = NameGeneratorBuilder::new()
        .bulk_add_names(place_names)
        .build();
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    (0..100).for_each(|_| {
        let name = generator.generate(|| rng.gen());
        println!("{} /{}/", format(name.0), format(name.1));
    });
}
