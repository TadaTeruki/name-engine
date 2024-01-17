use placename::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[test]
fn test_simple_generation() {
    let place_names = vec![
        // 青森
        PlaceName::new(vec![("青", "ao"), ("森", "mori")]),
        // 盛岡
        PlaceName::new(vec![("盛", "mori"), ("岡", "oka")]),
        // 酒田
        PlaceName::new(vec![("酒", "saka"), ("田", "ta")]),
        // 米沢
        PlaceName::new(vec![("米", "yone"), ("沢", "zawa")]),
        // 秋田
        PlaceName::new(vec![("秋", "aki"), ("田", "ta")]),
        // 山形
        PlaceName::new(vec![("山", "yama"), ("形", "gata")]),
        // 福島
        PlaceName::new(vec![("福", "fuku"), ("島", "shima")]),
        // 仙台
        PlaceName::new(vec![("仙", "sen"), ("台", "dai")]),
        // 長岡
        PlaceName::new(vec![("長", "naga"), ("岡", "oka")]),
        // 上越
        PlaceName::new(vec![("上", "jo"), ("越", "etsu")]),
        // 佐渡
        PlaceName::new(vec![("佐", "sa"), ("渡", "do")]),
        // 高崎
        PlaceName::new(vec![("高", "taka"), ("崎", "saki")]),
        // 日立
        PlaceName::new(vec![("日", "hi"), ("立", "tachi")]),
        // 水戸
        PlaceName::new(vec![("水", "mi"), ("戸", "to")]),
        // 成田
        PlaceName::new(vec![("成", "nari"), ("田", "ta")]),
        // 佐倉
        PlaceName::new(vec![("佐", "sa"), ("倉", "kura")]),
        // 市原
        PlaceName::new(vec![("市", "ichi"), ("原", "hara")]),
        // 立川
        PlaceName::new(vec![("立", "tachi"), ("川", "kawa")]),
        // 所沢
        PlaceName::new(vec![("所", "tokoro"), ("沢", "zawa")]),
        // 飯田
        PlaceName::new(vec![("飯", "ii"), ("田", "da")]),
        // 長野
        PlaceName::new(vec![("長", "naga"), ("野", "no")]),
        // 松本
        PlaceName::new(vec![("松", "matsu"), ("本", "moto")]),
        // 豊橋
        PlaceName::new(vec![("豊", "toyo"), ("橋", "hashi")]),
        // 岡崎
        PlaceName::new(vec![("岡", "oka"), ("崎", "zaki")]),
        // 豊田
        PlaceName::new(vec![("豊", "toyo"), ("田", "ta")]),
        // 高山
        PlaceName::new(vec![("高", "taka"), ("山", "yama")]),
        // 金沢
        PlaceName::new(vec![("金", "kana"), ("沢", "zawa")]),
        // 富山
        PlaceName::new(vec![("富", "to"), ("山", "yama")]),
        // 福井
        PlaceName::new(vec![("福", "fuku"), ("井", "i")]),
        // 甲府
        PlaceName::new(vec![("甲", "ko"), ("府", "fu")]),
        // 静岡
        PlaceName::new(vec![("静", "shizu"), ("岡", "oka")]),
        // 浜松
        PlaceName::new(vec![("浜", "hama"), ("松", "matsu")]),
        // 長浜
        PlaceName::new(vec![("長", "naga"), ("浜", "hama")]),
        // 小浜
        PlaceName::new(vec![("小", "o"), ("浜", "bama")]),
        // 大津
        PlaceName::new(vec![("大", "o"), ("津", "tsu")]),
        // 亀岡
        PlaceName::new(vec![("亀", "kame"), ("岡", "oka")]),
        // 豊岡
        PlaceName::new(vec![("豊", "toyo"), ("岡", "oka")]),
        // 三田
        PlaceName::new(vec![("三", "san"), ("田", "da")]),
        // 松原
        PlaceName::new(vec![("松", "matsu"), ("原", "bara")]),
        // 有田
        PlaceName::new(vec![("有", "ari"), ("田", "ta")]),
        // 徳島
        PlaceName::new(vec![("徳", "toku"), ("島", "shima")]),
        // 高松
        PlaceName::new(vec![("高", "taka"), ("松", "matsu")]),
        // 松山
        PlaceName::new(vec![("松", "matsu"), ("山", "yama")]),
        // 南国
        PlaceName::new(vec![("南", "nan"), ("国", "koku")]),
        // 岡山
        PlaceName::new(vec![("岡", "oka"), ("山", "yama")]),
        // 広島
        PlaceName::new(vec![("広", "hiro"), ("島", "shima")]),
        // 福山
        PlaceName::new(vec![("福", "fuku"), ("山", "yama")]),
        // 赤磐
        PlaceName::new(vec![("赤", "aka"), ("磐", "iwa")]),
        // 津山
        PlaceName::new(vec![("津", "tsu"), ("山", "yama")]),
        // 岩国
        PlaceName::new(vec![("岩", "iwa"), ("国", "kuni")]),
        // 山口
        PlaceName::new(vec![("山", "yama"), ("口", "guchi")]),
        // 宇部
        PlaceName::new(vec![("宇", "u"), ("部", "be")]),
        // 浜田
        PlaceName::new(vec![("浜", "hama"), ("田", "da")]),
        // 大田
        PlaceName::new(vec![("大", "o"), ("田", "ta")]),
        // 松江
        PlaceName::new(vec![("松", "matsu"), ("江", "e")]),
        // 倉吉
        PlaceName::new(vec![("倉", "kura"), ("吉", "yoshi")]),
        // 佐世保
        PlaceName::new(vec![("佐", "sa"), ("世", "se"), ("保", "bo")]),
        // 福岡
        PlaceName::new(vec![("福", "fuku"), ("岡", "oka")]),
        // 大村
        PlaceName::new(vec![("大", "o"), ("村", "mura")]),
        // 島原
        PlaceName::new(vec![("島", "shima"), ("原", "bara")]),
        // 中津
        PlaceName::new(vec![("中", "naka"), ("津", "tsu")]),
        // 玉名
        PlaceName::new(vec![("玉", "tama"), ("名", "na")]),
        // 人吉
        PlaceName::new(vec![("人", "hito"), ("吉", "yoshi")]),
        // 霧島
        PlaceName::new(vec![("霧", "kiri"), ("島", "shima")]),
        // 日置
        PlaceName::new(vec![("日", "hi"), ("置", "oki")]),
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
