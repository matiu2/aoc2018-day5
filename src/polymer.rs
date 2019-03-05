/// A polymer is a string of letters
/// If two letters, the same, but different case are next to each other they'll cancel each other out
/// These reactions will continue again on the new resultant string until no more can be cancelled out

pub struct Polymer {
    pub data: Vec<u8>,
}

impl Polymer {
    // Perform one reaction
    pub fn react(&mut self) -> bool {
        let mut i = 0;
        let max = self.data.len();
        let mut indexes = Vec::new();
        while i < max {
            // skip over '.'s
            while i < max && self.data[i] == b'.' {
                i += 1;
            }
            if i == max {
                break;
            }
            let c1 = self.data[i];
            let i1 = i;
            i += 1;
            // skip over '.'s
            while i < max && self.data[i] == b'.' {
                i += 1;
            }
            if i == max {
                break;
            }
            let c2 = self.data[i];
            let i2 = i;
            #[cfg(test)]
            println!("checking pair: {}{}", c1 as char, c2 as char);
            // This pair self destructs..
            if (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
                && ((c1.is_ascii_uppercase() && c2.is_ascii_lowercase())
                    || c1.is_ascii_lowercase() && c2.is_ascii_uppercase())
            {
                // Block out these two chars
                self.data[i1] = b'.';
                self.data[i2] = b'.';
                indexes.push(i1);
                indexes.push(i2);
                // Go back to before the pair
                if i1 > 0 {
                    i = i1 - 1;
                } else {
                    i = i2 + 1;
                    if i == max {
                        break;
                    }
                }
                #[cfg(test)]
                println!("After change: {}", String::from_utf8_lossy(&self.data));
            }
        }
        // Remove all the evil indexes
        if indexes.len() > 0 {
            indexes.sort();
            indexes.into_iter().rev().for_each(|i| {
                assert_eq!(self.data[i], b'.');
                self.data.remove(i);
            });
            true
        } else {
            false
        }
    }
}

#[test]
fn test_react1() {
    let mut data = Polymer {
        data: "dabAcCaCBAcCcaDA".into(),
    };
    println!("Start: {}", String::from_utf8_lossy(&data.data));
    while data.react() {}
    let got = String::from_utf8_lossy(&data.data);
    assert_eq!(got, "dabCBAcaDA");
}

#[test]
fn test_react2() {
    let mut data = Polymer {
        data: "VRrQqmPoOpMVq".into(),
    };
    println!("Start: {}", String::from_utf8_lossy(&data.data));
    while data.react() {}
    let got = String::from_utf8_lossy(&data.data);
    assert_eq!(got, "VVq");
}
