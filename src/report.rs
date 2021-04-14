pub struct Report {
    n_removed_tweets: u32,
}

impl Report {
    pub fn new(n_removed_tweets: u32) -> Self {
        Report {
            n_removed_tweets
        }
    }

    pub fn get_number_of_removed_tweets(&self) -> u32 {
        self.n_removed_tweets
    }
}
