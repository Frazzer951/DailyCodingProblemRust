/* EASY
Implement a URL shortener with the following methods:

 * shorten(url), which shortens the url into a six-character alphanumeric
   string, such as zLg6wl.
 * restore(short), which expands the shortened string into the original url. If
   no such shortened string exists, return null.

Hint: What if we enter the same URL twice?
*/

use std::collections::HashMap;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

struct Shortener {
    encode_map: HashMap<String, String>,
    decode_map: HashMap<String, String>,
}

impl Shortener {
    pub fn new() -> Self {
        Self {
            encode_map: Default::default(),
            decode_map: Default::default(),
        }
    }

    pub fn shorten(&mut self, url: String) -> String {
        if self.encode_map.contains_key(&url) {
            // return encoded key for url
            return self.encode_map.get(&url).unwrap().to_string();
        }
        loop {
            // Keep generating random short strings until a new one is made
            let short = Shortener::gen_string();
            if !self.decode_map.contains_key(&short) {
                self.encode_map.insert(url.clone(), short.clone());
                self.decode_map.insert(short.clone(), url);
                return short;
            }
        }
    }

    pub fn restore(&self, short: String) -> Option<String> {
        self.decode_map.get(&short).map(|short| short.to_string())
    }

    fn gen_string() -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortener() {
        let mut shortener = Shortener::new();

        let urls = vec![
            "www.google.com".to_string(),
            "www.github.com".to_string(),
            "www.dailycodingproblem.com".to_string(),
        ];
        let mut shorts = vec![];

        for url in &urls {
            shorts.push(shortener.shorten(url.clone()));
        }

        for (i, short) in shorts.iter().enumerate() {
            let decode = shortener.restore(short.clone());
            assert_eq!(decode.unwrap(), urls[i]);
        }
    }

    #[test]
    fn test_shortener_no_url() {
        let shortener = Shortener::new();

        assert_eq!(shortener.restore("000000".to_string()), None);
    }

    #[test]
    fn test_shortener_same_url() {
        let mut shortener = Shortener::new();

        let short = shortener.shorten("www.google.com".to_string());

        assert_eq!(shortener.shorten("www.google.com".to_string()), short);
    }
}
