/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use placename_engine::{PlaceName, PlaceNameGeneratorBuilder};
use rand::{rngs::StdRng, Rng, SeedableRng};

fn main() {
    let place_names = vec![
        // 青森
        vec![("青", "ao"), ("森", "mori")],
        // 盛岡
        vec![("盛", "mori"), ("岡", "oka")],
        // 酒田
        vec![("酒", "saka"), ("田", "ta")],
        // 米沢
        vec![("米", "yone"), ("沢", "zawa")],
        // 秋田
        vec![("秋", "aki"), ("田", "ta")],
        // 山形
        vec![("山", "yama"), ("形", "gata")],
        // 福島
        vec![("福", "fuku"), ("島", "shima")],
        // 仙台
        vec![("仙", "sen"), ("台", "dai")],
        // 長岡
        vec![("長", "naga"), ("岡", "oka")],
        // 上越
        vec![("上", "jo"), ("越", "etsu")],
        // 佐渡
        vec![("佐", "sa"), ("渡", "do")],
        // 高崎
        vec![("高", "taka"), ("崎", "saki")],
        // 伊勢崎
        vec![("伊", "i"), ("勢", "se"), ("崎", "saki")],
        // 日立
        vec![("日", "hi"), ("立", "tachi")],
        // 水戸
        vec![("水", "mi"), ("戸", "to")],
        // 成田
        vec![("成", "nari"), ("田", "ta")],
        // 船橋
        vec![("船", "funa"), ("橋", "bashi")],
        // 佐倉
        vec![("佐", "sa"), ("倉", "kura")],
        // 市原
        vec![("市", "ichi"), ("原", "hara")],
        // 立川
        vec![("立", "tachi"), ("川", "kawa")],
        // 八王子
        vec![("八", "hachi"), ("王", "o"), ("子", "ji")],
        // 所沢
        vec![("所", "tokoro"), ("沢", "zawa")],
        // 飯田
        vec![("飯", "ii"), ("田", "da")],
        // 上田
        vec![("上", "jo"), ("田", "da")],
        // 小諸
        vec![("小", "ko"), ("諸", "moro")],
        // 長野
        vec![("長", "naga"), ("野", "no")],
        // 松本
        vec![("松", "matsu"), ("本", "moto")],
        // 豊橋
        vec![("豊", "toyo"), ("橋", "hashi")],
        // 名古屋
        vec![("名", "na"), ("古", "go"), ("屋", "ya")],
        // 岡崎
        vec![("岡", "oka"), ("崎", "zaki")],
        // 豊田
        vec![("豊", "toyo"), ("田", "ta")],
        // 高山
        vec![("高", "taka"), ("山", "yama")],
        // 金沢
        vec![("金", "kana"), ("沢", "zawa")],
        // 富山
        vec![("富", "to"), ("山", "yama")],
        // 福井
        vec![("福", "fuku"), ("井", "i")],
        // 甲府
        vec![("甲", "ko"), ("府", "fu")],
        // 静岡
        vec![("静", "shizu"), ("岡", "oka")],
        // 浜松
        vec![("浜", "hama"), ("松", "matsu")],
        // 長浜
        vec![("長", "naga"), ("浜", "hama")],
        // 小浜
        vec![("小", "o"), ("浜", "bama")],
        // 大津
        vec![("大", "o"), ("津", "tsu")],
        // 亀岡
        vec![("亀", "kame"), ("岡", "oka")],
        // 豊岡
        vec![("豊", "toyo"), ("岡", "oka")],
        // 神戸
        vec![("神", "ko"), ("戸", "be")],
        // 姫路
        vec![("姫", "hime"), ("路", "ji")],
        // 洲本
        vec![("洲", "su"), ("本", "moto")],
        // 三田
        vec![("三", "san"), ("田", "da")],
        // 松原
        vec![("松", "matsu"), ("原", "bara")],
        // 岸和田
        vec![("岸", "kishi"), ("和", "wa"), ("田", "ta")],
        // 有田
        vec![("有", "ari"), ("田", "ta")],
        // 徳島
        vec![("徳", "toku"), ("島", "shima")],
        // 高松
        vec![("高", "taka"), ("松", "matsu")],
        // 松山
        vec![("松", "matsu"), ("山", "yama")],
        // 南国
        vec![("南", "nan"), ("国", "koku")],
        // 岡山
        vec![("岡", "oka"), ("山", "yama")],
        // 広島
        vec![("広", "hiro"), ("島", "shima")],
        // 福山
        vec![("福", "fuku"), ("山", "yama")],
        // 赤磐
        vec![("赤", "aka"), ("磐", "iwa")],
        // 津山
        vec![("津", "tsu"), ("山", "yama")],
        // 岩国
        vec![("岩", "iwa"), ("国", "kuni")],
        // 山口
        vec![("山", "yama"), ("口", "guchi")],
        // 宇部
        vec![("宇", "u"), ("部", "be")],
        // 浜田
        vec![("浜", "hama"), ("田", "da")],
        // 大田
        vec![("大", "o"), ("田", "ta")],
        // 松江
        vec![("松", "matsu"), ("江", "e")],
        // 倉吉
        vec![("倉", "kura"), ("吉", "yoshi")],
        // 佐世保
        vec![("佐", "sa"), ("世", "se"), ("保", "bo")],
        // 福岡
        vec![("福", "fuku"), ("岡", "oka")],
        // 大村
        vec![("大", "o"), ("村", "mura")],
        // 島原
        vec![("島", "shima"), ("原", "bara")],
        // 中津
        vec![("中", "naka"), ("津", "tsu")],
        // 久留米
        vec![("久", "ku"), ("留", "ru"), ("米", "me")],
        // 大牟田
        vec![("大", "o"), ("牟", "mu"), ("田", "ta")],
        // 玉名
        vec![("玉", "tama"), ("名", "na")],
        // 人吉
        vec![("人", "hito"), ("吉", "yoshi")],
        // 霧島
        vec![("霧", "kiri"), ("島", "shima")],
        // 日置
        vec![("日", "hi"), ("置", "oki")],
        // 鹿児島
        vec![("鹿", "ka"), ("児", "go"), ("島", "shima")],
        // 宮崎
        vec![("宮", "miya"), ("崎", "zaki")],
    ]
    .iter()
    .map(|placename| PlaceName::new(placename.to_vec()).unwrap())
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
