use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq)]
struct Record {
    name: String,
    wins: u32,
    draws: u32,
    loses: u32,
    games: u32,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.points().cmp(&other.points()) {
            Ordering::Equal => self.name.cmp(&other.name),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Record {
    fn new(name: String) -> Self {
        return Self {
            name: name,
            wins: 0,
            loses: 0,
            draws: 0,
            games: 0,
        };
    }

    fn points(&self) -> u32 {
        (self.wins * 3) + self.draws
    }

    fn won(&mut self) {
        self.wins += 1;
        self.games += 1;
    }

    fn lost(&mut self) {
        self.loses += 1;
        self.games += 1;
    }

    fn tied(&mut self) {
        self.draws += 1;
        self.games += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut results = format!("{:<31}| MP |  W |  D |  L |  P", "Team");
    let mut records: HashMap<String, Record> = HashMap::new();

    if match_results.len() < 1 {
        return results;
    }

    for line in match_results.split("\n") {
        let game: Vec<&str> = line.split(";").collect();
        let (home_team_name, away_team_name, outcome) = (game[0], game[1], game[2]);

        records
            .entry(home_team_name.to_owned())
            .or_insert(Record::new(home_team_name.to_owned()));

        records
            .entry(away_team_name.to_owned())
            .or_insert(Record::new(away_team_name.to_owned()));

        match outcome {
            "win" => {
                records
                    .entry(home_team_name.to_owned())
                    .and_modify(|team| team.won());
                records
                    .entry(away_team_name.to_owned())
                    .and_modify(|team| team.lost());
            }
            "loss" => {
                records
                    .entry(home_team_name.to_owned())
                    .and_modify(|team| team.lost());
                records
                    .entry(away_team_name.to_owned())
                    .and_modify(|team| team.won());
            }
            "draw" => {
                records
                    .entry(home_team_name.to_owned())
                    .and_modify(|team| team.tied());
                records
                    .entry(away_team_name.to_owned())
                    .and_modify(|team| team.tied());
            }
            _ => panic!("wtf"),
        };
    }

    let mut by_points_and_name: Vec<&Record> = records.values().collect();

    by_points_and_name.sort();

    for record in by_points_and_name {
        results.push_str(
            &format!(
                "\n{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
                record.name,
                record.games,
                record.wins,
                record.draws,
                record.loses,
                record.points()
            )
            .to_owned(),
        );
    }

    return results;
}
