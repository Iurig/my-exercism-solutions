#[derive(Debug)]
pub struct HighScores {
    all_scores: Vec<u32>,
    top_3: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let vec_scores = scores.to_vec();
        let mut last_three_scores = vec_scores.clone();
        last_three_scores.sort_unstable();
        last_three_scores.reverse();
        last_three_scores.truncate(3);
        HighScores {
            all_scores: vec_scores,
            top_3: last_three_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.all_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.all_scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.top_3.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top_3.clone()
    }
}
