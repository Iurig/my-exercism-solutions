use itertools::Itertools;

#[derive(Debug)]
pub struct HighScores<'a> {
    all_scores: &'a [u32],
    top_3: Vec<u32>,
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let vec_scores = scores;
        let last_three_scores = vec_scores.iter().k_largest(3).cloned().collect_vec();
        HighScores::<'a> {
            all_scores: vec_scores,
            top_3: last_three_scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.all_scores
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
