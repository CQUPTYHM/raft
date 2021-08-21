use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    Leader,
    Follower,
    Candidater(u32),
}

#[derive(Serialize, Deserialize)]
pub enum Message {
    Log(AppendEntry),
    HeartBeat(AppendEntry),
    AppendEntryReply(AppendEntryReply),
    RequestVote(RequestVote),
    //(bool, term)
    VoteFor(Vote),
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppendEntry {
    pub term: u32,
    pub leader_addr: String,
    pub pre_log_index: u32,
    pub pre_log_term: u32,
    pub entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppendEntryReply {
    pub term: u32,
    pub addr: String,
    pub success: bool,
    pub apply_index: u32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Entry {
    term: u32,
    data: Vec<u8>,
}

pub struct Reply {
    term: u32,
    is_success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RequestVote {
    pub term: u32,
    pub addr: String,
    pub cur_index: u32,
    pub cur_term: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Vote {
    pub term: u32,
    pub vote_granted: bool,
}

pub struct State {
    pub rule: Rule,
    pub term: u32,
    pub vote_for: Option<String>,
    //(term, index, log)
    pub log: Vec<(u32, u32, AppendEntry)>,
    pub leader_id: Option<u32>,
    pub commit_index: u32,

    //Volatile state on leaders
    pub next_index: Option<Vec<u32>>,
    pub match_index: Option<Vec<u32>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rule: Rule::Follower,
            term: 0u32,
            vote_for: None,
            log: Vec::new(),
            leader_id: None,
            commit_index: 0u32,

            next_index: None,
            match_index: None,
        }
    }
}
