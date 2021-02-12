use {
    crate::*,
};

/// A non empty group of lines, with a common characteristic,
/// for stats
pub struct LineGroup<'b> {
    pub lines: Vec<&'b LogLine>, // guaranteed not empty
    pub trend: Trend,
    pub bytes: u64,
    pub key_sum: u64,
}

impl<'b> LineGroup<'b> {
    pub fn new(
        lines: Vec<&'b LogLine>,
        trend_computer: &TrendComputer,
    ) -> Self {
        debug_assert!(!lines.is_empty());
        let trend = trend_computer.compute_trend(&lines);
        let bytes = lines.iter().map(|ll| ll.bytes_sent).sum();
        let key_sum = match trend_computer.key {
            Key::Hits => lines.len() as u64,
            Key::Bytes => bytes,
        };
        Self {
            lines,
            trend,
            bytes,
            key_sum,
        }
    }
    pub fn any(&self) -> &LogLine {
        &self.lines[0]
    }
    pub fn hits(&self) -> usize {
        self.lines.len()
    }
    pub fn histo_line(&self) -> String {
        histo_line(
            &self.trend.sum_per_day,
            self.trend.max_day_count(),
            false,
        )
    }
}
