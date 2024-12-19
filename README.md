# Kullanışlı Rust Kütüphaneleri

Bu repoda aşağıdaki liste dahilinde yararlı olduğunu düşündüğüm Rust küfelerini incelemeyi planlıyorum. Rust ekosisteminin güçlü yönlerinden birisi de topluluk desteği ve kütüphane zenginliğidir. Burada incelemek istediğim Crate'ler haricinde sayısız kütüphane vardır. [Crates IO](https://crates.io/) adresinden daha fazlasına bakılabilir.

## Proje Kodlama Standartları

- Projeler **try_[CRATE_NAME]** standardında isimlendirilmektedir. try_clap, try_actix vb.
- Senaryoların crate kullanılmadan işletilen versiyonları genellikle **bare.rs** dosyasında tutulur. Crate kullanılan sürümler ise **equiped.rs** dosyasına konur. Burada amaç crate kullanımının neleri kolaylaştırdığını göstermektir.
- Geliştirilen program ile ilgili bilgiler main fonksiyonunun başında aşağıdaki standartlara benzer şekilde tutulur.

```text
Program : En Sevdiğim Filmler
Amaç    : clap crate kullanımının örneklenmesi

Senaryo : Sevdiğim filmlerin adını, hangi yıl çıktığını ve bana göre 10 üzerinden puanlamasını tutmak istediğim 
bir terminalden çalışan program. Film bilgileri csv türünden bir dosyada tutulacak.

Fonksiyonellikler :

Film Ekleme
Film Listeleme
Film Silme

Detaylar :

Tüm fonksiyonellikler komut satırından desteklenir. Örnek komutlar,

add matrix 1999 9.5
list
remove matrix

Çalıştırma :

cargo run -- add matrix 1999 9.5
cargo run -- list
cargo run -- remove matrix
```

## Crate List

Ele alınan/alınacak rust kütüphaneleri aşağıdaki gibidir.

| Id | Crate               | Kategori                   | Ek Bilgiler |
|-----|---------------------|--------------------------|---------------------------------|
| **00** | [clap](https://crates.io/crates/clap)              | Terminal                     | |
| **01** | [rocket](https://crates.io/search?q=rocket)| Async Web Framework                     | |
| **02** | [diesel](https://crates.io/crates/diesel)| orm                     | |
| **03** | [actix](https://crates.io/crates/actix)| Actor Based Web Framework                     | |
| **04** | [hyper](https://crates.io/crates/hyper)| Low Level Http Library                      | |
| **05** | [tokio](https://crates.io/crates/tokio)| Async Runtime                 | |
| **06** | [rayon](https://crates.io/crates/rayon)| Data Parallelism                      | |
| **07** | [serde](https://crates.io/crates/serde)| Generic Serialization/Deserialization                      | |
| **08** | [log](https://crates.io/crates/log)| Logging                     | |
| **09** | [anyhow](https://crates.io/crates/anyhow)| Ideomatic Error Handling                      | |
| **10** | [thiserror](https://crates.io/crates/thiserror)| Error Handling                      | |
| **11** | [validator](https://crates.io/crates/validator)| Validation                      | |
| **12** | [derive_builder](https://crates.io/crates/derive_builder)| Builder                      | |
| **13** | [chrono](https://crates.io/crates/chrono)| Date Time Operations                      | 
| **14** | [soa_dervie](https://crates.io/crates/soa_derive)| Code Generation |Array of structs(AOS), Struct of Arrays (SOA) |
| **15** | [criterion](https://crates.io/crates/criterion)|Benchmark                      | |
| **16** | [regex-syntax](https://crates.io/crates/regex-syntax)|Regular Expressions                      | |
| **17** | [humantime](https://crates.io/crates/humantime)| Parser                      | |
| **18** | [serde_yml](https://crates.io/crates/serde_yml) | Serialization  | |
