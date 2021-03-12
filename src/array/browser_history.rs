//! Modeling A Browser's History
//!
//! The goal is to model a browser's history, where you can perform three different actions:
//!
//!     1. Visit a url.
//!     2. Move back to visit any previously visited url
//!     3. Move forward up until the most recently visited url.
//!
//! A browser should be initialized with a *fixed* capacity and homepage.
//!
//! **Note:** Care must be taken to ignore a browser's "forward" history when visiting a new url.
//! For example, suppose a browser's history consists of sites "A", "B", "C". If we move back one
//! step to site "A" and then vist a new site "D", the new history should be "A", "B", "D"


pub struct BrowserHistory {
    capacity: usize,
    homepage: String,
    history: Vec<String>,

    // Internal ptrs
    current: usize,
    newest: usize,
    oldest: usize,
}


// Constructors & properties
impl BrowserHistory {
    pub fn new(capacity: usize, homepage: String) -> BrowserHistory {
        let mut history = vec![String::new(); capacity];
        history[0] = homepage.clone();

        BrowserHistory {
            capacity,
            homepage,
            history,

            current: 0,
            newest: 0,
            oldest: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn homepage(&self) -> &str {
        &self.homepage[..]
    }

    pub fn history(&self) -> &[String] {
        &self.history[..]
    }
}


// Methods
impl BrowserHistory {
    pub fn visit(&mut self, url: String) {
        self.current = (self.current + 1) % self.capacity;
        self.history[self.current] = url;
        self.newest = self.current;

        if self.oldest == self.newest {
            self.oldest = (self.oldest + 1) % self.capacity;
        }
    }

    pub fn back(&mut self, steps: usize) {
        if self.current >= self.oldest {
            self.current -= steps.min(self.current - self.oldest);
        } else {
            let (cur, old, cap) = (self.current as i32, self.oldest as i32, self.capacity as i32);
            let current = cur - (steps as i32).min(cur + cap - old);
            self.current = current.rem_euclid(cap) as usize;
        }
    }

    pub fn forward(&mut self, steps: usize) {
        if self.current <= self.newest {
            self.current += steps.min(self.newest - self.current);
        } else {
            self.current += steps.min(self.capacity - self.current + self.newest);
            self.current = self.current.rem_euclid(self.capacity);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Returns an instance of BrowserHistory with full capacity and history: ["H", "1", "2", ...]
    fn browser_with_full_capacity(capacity: usize) -> BrowserHistory {
        let mut browser = BrowserHistory::new(capacity, "H".to_string());
        for i in 1 .. capacity {
            browser.visit(i.to_string());
        }
        browser
    }

    #[test]
    /// Browser at full capacity overwrites oldest url with the newly visited one
    fn history_wraps_at_full_capacity() {
        let mut browser = browser_with_full_capacity(3);  // ["H", "1", "2"]
        browser.visit("3".to_string());
        assert_eq!(browser.oldest, 1);
        assert_eq!(browser.newest, 0);
        assert_eq!(browser.current, 0);
        assert_eq!(browser.history, vec!["3", "1", "2"]);
    }

    #[test]
    /// Moving back then visiting a new url resets `newest` (hence ignores forward history)
    fn back_then_visit_resets_newest_ptr() {
        let mut browser = browser_with_full_capacity(4);  // ["H", "1", "2", "3"]
        browser.back(2);
        browser.visit("4".to_string());
        assert_eq!(browser.newest, 2);
        assert_eq!(browser.current, 2);
        assert_eq!(browser.history, vec!["H", "1", "4", "3"]);  // Our ptrs ignore "3"
    }

    #[test]
    /// Moving back then forward by the same number of steps is a noop
    fn forward_undos_backward() {
        let mut browser = browser_with_full_capacity(3);
        browser.back(2);
        browser.forward(2);
        assert_eq!(browser.newest, 2);
        assert_eq!(browser.current, 2);
    }

    #[test]
    fn back_does_not_move_current_before_oldest() {
        let mut browser = browser_with_full_capacity(3);
        browser.back(6);
        assert_eq!(browser.oldest, 0);
        assert_eq!(browser.current, 0);
    }

    #[test]
    fn forward_does_not_move_current_past_newest() {
        let mut browser = browser_with_full_capacity(3);
        browser.back(1);
        browser.forward(5);
        assert_eq!(browser.newest, 2);
        assert_eq!(browser.current, 2);
    }

    #[test]
    fn current_at_index_zero_becomes_last_index_after_calling_back_one() {
        let mut browser = browser_with_full_capacity(3);
        browser.visit("3".to_string());
        browser.back(1);
        assert_eq!(browser.current, 2);
    }

    #[test]
    fn back_two_forward_one_moves_current_one_left_of_newest() {
        let mut browser = browser_with_full_capacity(5);
        browser.back(2);
        browser.forward(1);
        assert_eq!(browser.current, browser.newest - 1);
    }
}
