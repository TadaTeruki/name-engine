use placename::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn test_simple_generation() {
    let place_names = vec![
        PlaceName::new(vec![("北", "Bei"), ("京", "jing")]),
        PlaceName::new(vec![("天", "Tian"), ("津", "jin")]),
        PlaceName::new(vec![("重", "Chong"), ("庆", "qing")]),
        PlaceName::new(vec![("上", "Shang"), ("海", "hai")]),
        PlaceName::new(vec![("广", "Guang"), ("州", "zhou")]),
        PlaceName::new(vec![("深", "Shen"), ("圳", "zhen")]),
        PlaceName::new(vec![("哈", "Ha"), ("尔", "er"), ("滨", "bin")]),
        PlaceName::new(vec![("长", "Chang"), ("春", "chun")]),
        PlaceName::new(vec![("大", "Da"), ("连", "lian")]),
        PlaceName::new(vec![("鞍", "An"), ("山", "shan")]),
        PlaceName::new(vec![("沈", "Shen"), ("阳", "yang")]),
        PlaceName::new(vec![("保", "Bao"), ("定", "ding")]),
        PlaceName::new(vec![("石", "Shi"), ("家", "jia"), ("庄", "zhuang")]),
        PlaceName::new(vec![("大", "Da"), ("同", "tong")]),
        PlaceName::new(vec![("包", "Bao"), ("头", "tou")]),
        PlaceName::new(vec![("太", "Tai"), ("原", "yuan")]),
        PlaceName::new(vec![("成", "Cheng"), ("都", "du")]),
        PlaceName::new(vec![("杭", "Hang"), ("州", "zhou")]),
        PlaceName::new(vec![("苏", "Su"), ("州", "zhou")]),
        PlaceName::new(vec![("南", "Nan"), ("京", "jing")]),
        PlaceName::new(vec![("西", "Xi"), ("安", "an")]),
        PlaceName::new(vec![("武", "Wu"), ("汉", "han")]),
        PlaceName::new(vec![("长", "Chang"), ("沙", "sha")]),
        PlaceName::new(vec![("青", "Qing"), ("岛", "dao")]),
        PlaceName::new(vec![("厦", "Xia"), ("门", "men")]),
        PlaceName::new(vec![("福", "Fu"), ("州", "zhou")]),
        PlaceName::new(vec![("济", "Ji"), ("南", "nan")]),
        PlaceName::new(vec![("昆", "Kun"), ("明", "ming")]),
        PlaceName::new(vec![("兰", "Lan"), ("州", "zhou")]),
        PlaceName::new(vec![("贵", "Gui"), ("阳", "yang")]),
    ];

    let generator = PlaceNameGeneratorBuilder::new()
        .bulk_add_place_names(place_names)
        .build();
    let mut rng = StdRng::seed_from_u64(0);
    (0..1000).for_each(|_| {
        let name = generator.generate(|| rng.gen());
        println!("{} {}", name.0, name.1);
    });
}
