struct Codec {
    ptr: usize,
    urls: Vec<String>,
}

impl Codec {
    fn new() -> Self {
        Self {
            ptr: 0,
            urls: Vec::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, longURL: String) -> String {
        // push long url to urls
        self.urls.push(longURL);

        // increment ptr
        self.ptr += 1;

        // return the old pointer as string
        (self.ptr - 1).to_string()
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, shortURL: String) -> String {
        // parse shortURL as usize
        let ptr = shortURL.parse::<usize>().unwrap();

        // get long url at this index
        self.urls[ptr].to_string()
    }
}
