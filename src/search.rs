// search - find and replace functionality
/// find and replace state
#[derive(Default)]
pub struct SearchState {
    pub query: String,
    pub replace: String,
    pub case_sensitive: bool,
    pub matches: Vec<usize>, // byte positions of matches
    pub current_match: usize,
}

// search helpers
impl SearchState {
    // find all matches of the query in the text
    pub fn update_matches(&mut self, text: &str) {
        self.matches.clear();
        self.current_match = 0;

        if self.query.is_empty() {
            return;
        }

        let haystack = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };
        let needle = if self.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = haystack[start..].find(&needle) {
            let abs_pos = start + pos;
            self.matches.push(abs_pos);
            start = abs_pos + needle.len().max(1);
        }
    }

    // go to the next match
    pub fn next_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.matches.len();
        }
    }

    // go to the previous match
    pub fn prev_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = self
                .current_match
                .checked_sub(1)
                .unwrap_or(self.matches.len() - 1);
        }
    }
}
