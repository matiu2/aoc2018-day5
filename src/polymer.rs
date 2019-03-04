/// A polymer is a string of letters
/// If two letters, the same, but different case are next to each other they'll cancel each other out
/// These reactions will continue again on the new resultant string until no more can be cancelled out

pub struct Polymer {
    pub data: Vec<u8>,
}

impl Polymer {
    // Perform one reaction
    pub fn react(&mut self) {
        let mut i = 0;
        let max = self.data.len() - 2;
        let mut out = Vec::new();
        out.reserve(self.data.len());
        while i <= max {
            let c1 = self.data[i];
            let c2 = self.data[i + 1];
            // This pair self destructs..
            if (c1.to_ascii_lowercase() == c2.to_ascii_lowercase())
                && ((c1.is_ascii_uppercase() && c2.is_ascii_lowercase())
                    || c1.is_ascii_lowercase() && c2.is_ascii_uppercase())
            {
                i += 2;
            } else {
                out.push(c1);
                i += 1
            }
        }
        // Just copy the last byte
        // If we found just one polymer, we just copy the rest of the string
        while i < self.data.len() {
            out.push(self.data[i]);
            i += 1;
        }
        self.data = out;
    }
}

#[test]
fn test_react() {
    let mut data = Polymer {
        data: "dabAcCaCBAcCcaDA".into(),
    };
    for stage in 1..5 {
        let show = String::from_utf8_lossy(&data.data);
        println!("stage {} - data: {}", stage, show);
        data.react();
    }
    assert_eq!(data.data, b"dabCBAcaDA".to_vec());
}
