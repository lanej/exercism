#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        return self.scores;
    }

    pub fn latest(&self) -> Option<u32> {
        return match self.scores().last() {
            Some(score) => Some(score.to_owned()),
            None => None,
        };
    }

    pub fn personal_best(&self) -> Option<u32> {
        return self.scores.iter().max().to_owned();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort_by(|a, b| b.cmp(a));
        scores.truncate(3);

        return scores;
    }
}