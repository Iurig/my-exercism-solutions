#[derive(Debug)]
pub struct HighScores(Vec<u32>, Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let vec_scores = Vec::from(scores);
        let mut last_three_scores = vec_scores.clone();
        last_three_scores.sort();
        last_three_scores.reverse();
        last_three_scores.truncate(3);
        HighScores(vec_scores, last_three_scores)
    }

    pub fn scores(&self) -> &[u32] {
        &self.0[..]
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.1.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.1.clone()
    }
}
