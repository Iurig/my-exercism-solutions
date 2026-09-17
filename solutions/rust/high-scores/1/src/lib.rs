#[derive(Debug)]
pub struct HighScores(Vec<u32>, Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let vec_scores = Vec::from(scores);
        let mut sorted_vec_scores = vec_scores.clone();
        sorted_vec_scores.sort();
        sorted_vec_scores.reverse();
        HighScores(
            vec_scores,
            Vec::from(&sorted_vec_scores[..3.min(sorted_vec_scores.len())]),
        )
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
