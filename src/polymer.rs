/// A polymer is a string of letters
/// If two letters, the same, but different case are next to each other they'll cancel each other out
/// These reactions will continue again on the new resultant string until no more can be cancelled out

pub struct Polymer {
    pub data: Vec<u8>,
}

impl Polymer {
    // It takes the byte to start reading and returns the last byte it changed
    // if it returns start, it means it didn't make any changes
    pub fn react(&mut self, start: usize) -> usize {
        let mut i = 0;
        let max = self.data.len() - 2;
        let mut out = Vec::new();
        let mut new_start = start;
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
                new_start = i;
                break;
            } else {
                out.push(c1);
                i += 1
            }
        }
        // If we found just one polymer, we just copy the rest of the string
        while i < self.data.len() {
            out.push(self.data[i]);
            i += 1;
        }
        self.data = out;
        new_start
    }
}

#[test]
fn test_react() {
    let mut data = Polymer {
        data: "dabAcCaCBAcCcaDA".into(),
    };
    let mut start = 0;
    for stage in 1..5 {
        let show = String::from_utf8_lossy(&data.data);
        println!("stage {} - start: {} data: {}", stage, start, show);
        start = data.react(start);
    }
    assert_eq!(data.data, b"dabCBAcaDA".to_vec());
}
