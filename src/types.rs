enum Rule {
    Leader,
    Follower,
    Candidater,
}

enum Message {
    Log(AppendEntry),
    HeartBeat(AppendEntry),
    Reply,
    RequestVote(RequestVote),
    VoteFor(bool),
}

struct AppendEntry {
    term: u32,
    leader_id: u32,
    pre_log_index: u32,
    pre_log_term: u32,
    entries: Vec<Entry>,
}

struct Entry {
    term: u32,
    data: Vec<u8>,
}

struct Reply {
    term: u32,
    is_success: bool,
}
struct RequestVote {
    term: u32,
    id: u32,
    cur_index: u32,
    cur_term: u32,
}

struct Vote {
    term: u32,
    vote_granted: bool,
}

struct State {
    rule: Rule,
    log: Vec<AppendEntry>,
    leader_id: Option<u32>,
    commit_index: u32,

    //Volatile state on leaders
    next_index: Option<Vec<u32>>,
    match_index: Option<Vec<u32>>,
}
