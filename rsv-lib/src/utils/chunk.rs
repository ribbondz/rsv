use crate::utils::column_stats::ColumnStats;
use crate::utils::reader::Task;
use crossbeam_channel::Sender;

pub struct ChunkResult {
    pub bytes: usize,
    pub stat: ColumnStats,
}

pub struct ChunkParser {
    sep: char,
    quote: char,
}

impl ChunkParser {
    pub fn new(sep: char, quote: char) -> Self {
        ChunkParser { sep, quote }
    }

    pub fn parse(&self, task: Task, tx: Sender<ChunkResult>, mut stat: ColumnStats) {
        for l in task.lines {
            stat.parse_line(&l, self.sep, self.quote)
        }

        tx.send(ChunkResult {
            bytes: task.bytes,
            stat,
        })
        .unwrap()
    }
}
