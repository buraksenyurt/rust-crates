/*
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
*/

mod bare;
mod equiped;
mod movie;

fn main() {
    
}
