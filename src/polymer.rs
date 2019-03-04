/// A polymer is a string of letters
/// If two letters, the same, but different case are next to each other they'll cancel each other out
/// These reactions will continue again on the new resultant string until no more can be cancelled out
use std::iter::Iterator;

pub struct Polymer {
    pub data: String,
}

impl Polymer {
    pub fn react(&self) -> Polymer {
        let data: String = self.data[0..].chars().step_by(2).zip(self.data[1..].chars().step_by(2))
            .filter(|(a, b)| {
                !((a.to_ascii_lowercase() == b.to_ascii_lowercase())
                    && ((a.is_ascii_lowercase() && b.is_ascii_uppercase())
                        || (a.is_ascii_uppercase() && b.is_ascii_lowercase())))
            })
            .flat_map(|(a, b)| vec![a, b])
            .collect();
        Polymer { data }
    }
}

#[test]
fn test_react() {
    let stage1 = Polymer {
        data: "dabAcCaCBAcCcaDA".into(),
    };
    println!("stage 1: {}", stage1.data);
    let stage2 = stage1.react();
    println!("stage 2: {}", stage2.data);
    let stage3 = stage2.react();
    println!("stage 3: {}", stage3.data);
    let stage4 = stage3.react();
    println!("stage 4: {}", stage4.data);
    assert_eq!(stage4.data, "dabCBAcaDA");
}
