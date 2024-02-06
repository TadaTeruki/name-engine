use placename::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let csv_file = include_str!("assets/hokkaido.csv");

    let place_names = csv_file
        .lines()
        .map(|line| {
            let first_quote = line.find('"').unwrap();
            let last_quote = line.rfind('"').unwrap();
            let phrases = line[first_quote + 1..last_quote]
                .split(',')
                .map(|phrase| {
                    let mut split = phrase.split(':');
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
