# name-engine

*Preview: Generating English Place Names `examples/england_evaluated.rs`*

![placename-generation](https://github.com/TadaTeruki/name-engine/assets/69315285/ce7b6b1c-a8ad-477a-9b10-92a27cb6df1c)

`name-engine` is a basic library for computing Markov chains to generate names based on their pronunciation.

This can be used for various purposes, but primarily for generating place names.

## Algorithm

This library computes Markov chains from a dataset of names. The names must be separated by certain user-defined rules, such as syllables. Each of the separated units is treated as a state of the Markov chain.

The transition is defined as the connection between the pronunciations. For example: 

- `ŋ` -> `w` in `Ringwood /ˈrɪŋwʊd/` [`(Ring /ˈrɪŋ/)` `(wood /wʊd/)`]

- `k` -> `ə` in `Beccles /ˈbɛkəlz/` [`(Becc /ˈbɛk/)` `(les /əlz/)`]

- `k` -> `ə` and `m` -> `s` in `Berkhamsted /ˈbɜːrkəmstɛd/` [`(Berk /ˈbɜːrk/)` `(ham /əm/)` `(sted /stɛd/)`]

With the data adove, the model can generate `Berkles` from `(Berk /ˈbɜːrk/)` and `(les /əlz/)` by tracking the transition `k` -> `ə`.

The probability of the transition is calculated from the frequency of the connection in the dataset.

## Features
This library does:
- **Create name generator** from dataset of separated names.
- Generate names using Markov chains.

This library DOES NOT:
- Read and parse data from a file.
- **Automatically separate original names according to specific rules, such as syllables.** You must prepare the dataset yourself.
- **Evaluate names.** If you want to generate better names, you must implement the evaluation function and filtering process by yourself.
- **Combine another parameters.** If you want to do, `NameGenerator::generate_verbose` is useful to implement it by yourself.

This library only does the minimal processing necessary to generate names. To create a more practical name generator, some additional processing like above will be required.

## Documentation

Run `cargo doc --open` to see the documentation.

If you want to try it out, see the examples in `examples/`. For the first step, `examples/japanese.rs` is suitable for reading.

## Installation

```sh
[dependencies]
name-engine = "0.1.0"
```

## Examples

#### Generate 100 place names of Hokkaido

```sh
$ cargo run --example hokkaido
```

```
中富 nakatomi
初威冠 shoikappu
上沢 kamizawa
```

#### Generate 100 place names of England

```sh
$ cargo run --example england
```

```
Stoneon /ˈstəʊnən/
Thatchingworth /ˈθætʃɪŋwɜːθ/
Brentgomley /ˈbrɛntɡʌmli/
```

#### Generate 100 place names of England (extracted better ones)

```sh
$ cargo run --example england_evaluated
```

```
Oltham Abbey /ˈoʊlθəm ˈæbi/
Downbury /ˈdaʊnbəri/
Farhead /ˈfɑːrhɛd/
```

#### Generate 100 place names of US (extracted better ones)

```sh
$ cargo run --example us_evaluated
```

```
Winfield /ˈwɪnfiːld/
Perton /ˈpɛrtən/
Kinbridge Falls /ˈkɪnbrɪdʒ fɔːlz/
```

### About the English and US place name data for the examples

For English and US place name data, some symbols are added for better results.
- [1] Spaces are replaced by `+` and treated as independent syllables.
- [2] For the syllable with capital letter, an asterisk `*` is added at the beginning of the pronunciation to become the first syllable of the name or the next syllable of `+`.
- [3] For the pronunciation of the previous syllable of `+`, an asterisk `*` is added at the end of the pronunciation to become the previous syllable of `+`.

**Example**
```
Tunbridge Wells /ˈtʌnbrɪdʒ ˈwɛlz/
(Tun, /*ˈtʌn/) (bridge, /brɪdʒ*/) (+, /+/) (Wells, /*ˈwɛlz/)
```
- `(Tun /ˈtʌn/)` -> `(Tun /*ˈtʌn/)` [2]
- `(bridge /brɪdʒ/)` -> `(bridge /brɪdʒ*/)` [3]
- `(+ /+/)` [1]
- `(Wells /ˈwɛlz/)` -> `(Wells /*ˈwɛlz/)` [2]

Moreover, some suffexes are treated as independent syllables, such as `minster` and `bridge`.

## Data Source

`examples/assets/hokkaido.csv`: Hokkaido Government Opendata CC-BY4.0（https://creativecommons.org/licenses/by/4.0/deed.ja）
Modified from the original data.

Source: https://www.pref.hokkaido.lg.jp/link/shichoson/aiueo.html

## License

This project is licensed under the Mozilla Public License v2.0. See the [LICENSE](LICENSE) file for details.

Note that if you use, copy or modify the code in `examples`, you do not need to worry about the copyleft restrictions. Feel free to use it!
