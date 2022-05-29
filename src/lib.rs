use itertools::Itertools;

pub struct Log {
    pub date: String,
    pub player_id: String,
    pub score: u32,
}

pub struct RankData {
    player_id: String,
    mean_score: u32,
}

pub struct Output {
    pub rank: u8,
    pub player_id: String,
    pub mean_score: u32,
}

pub fn get_top10_avg(logs: Vec<Log>) -> Vec<(u32, Vec<RankData>)> {
    logs
        .into_iter()
        .into_group_map_by(|elt| elt.player_id.to_owned())
        .into_iter()
        .map(|(id, log)| RankData {
            player_id: id,
            mean_score: (log.iter().fold(0.0, |acc, x| acc + x.score as f32) / log.len() as f32).round() as u32, // NOTE: 丸められる可能性あり
        })
        .into_group_map_by(|data| data.mean_score)
        .into_iter()
        .sorted_by(|a, b| Ord::cmp(&b.0, &a.0)) // 降順
        .take(10) // 合計スコア上位10個だけ取る
        .collect::<Vec<_>>()
}

pub fn get_top10_scorer(top10_avg: Vec<(u32, Vec<RankData>)>) -> Vec<Output> {
    let mut result: Vec<Output> = Vec::new();
    let mut rank = 1;
    for (mean_score, data_list) in top10_avg {
        let len = data_list.len();
        for data in data_list {
            result.push(Output {
                rank,
                player_id: data.player_id,
                mean_score,
            })
        }
        rank += len as u8;

        if rank > 10 {
            break;
        }
    }
    result
}