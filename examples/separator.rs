//! Demonstrates a repetition block with a separator.

use tempt::tempt;

enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    tempt! {
        weekend [ Saturday ][ Sunday ];

        fn is_weekend(&self) -> bool {
            matches!(self, #(Self::#weekend)(|)*)
        }
    }
}

fn main() {
    assert!(!Day::Monday.is_weekend());
    assert!(!Day::Tuesday.is_weekend());
    assert!(!Day::Wednesday.is_weekend());
    assert!(!Day::Thursday.is_weekend());
    assert!(!Day::Friday.is_weekend());
    assert!(Day::Saturday.is_weekend());
    assert!(Day::Sunday.is_weekend());
}
