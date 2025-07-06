use crate::utils::column_stats::ColumnStats;
use crate::utils::reader::Task;
use crossbeam_channel::Sender;

pub struct ChunkResult {
    pub bytes: usize,
    pub stat: ColumnStats,
}

pub fn parse_chunk(
    task: Task,
    tx: Sender<ChunkResult>,
    mut stat: ColumnStats,
    sep: char,
    quote: char,
) {
    for l in task.lines {
        stat.parse_line(&l, sep, quote)
    }

    tx.send(ChunkResult {
        bytes: task.bytes,
        stat,
    })
    .unwrap()
}
