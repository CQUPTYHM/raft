use crate::error::{Error, Result};
use crate::types::*;
use std::net::{Ipv4Addr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::time::Instant;
trait rpc {
    fn send_message(&mut self, addr: Ipv4Addr, message: &Message) -> Result<()>;
}

pub struct Server {
    listener: TcpListener,
    state: Mutex<State>,
    last_heart_beat_time: Mutex<Instant>,
}

impl Server {
    pub fn new() -> Result<Self> {
        let listener = TcpListener::bind("").unwrap();
        Ok(Self {
            listener: listener,
            state: Mutex::new(State::default()),
            last_heart_beat_time: Mutex::new(Instant::now()),
        })
    }
}
pub fn start(server: Arc<Server>) {
    for stream in server.listener.incoming() {
        match stream {
            Ok(stream) => {
                let server = server.clone();
                thread::spawn(|| process_message(server, stream));
            }
            _ => {}
        }
    }
}
pub fn new_election(server: Arc<Server>) {
    let mut state = server.state.lock().unwrap();
    state.rule = Rule::Candidater(1);
    state.term += 1;
    state.vote_for = Some("my ip address".to_owned());
    *server.last_heart_beat_time.lock().unwrap() = Instant::now();
    start_election_timer(server.clone(), Duration::from_millis(100));
}

pub fn start_election_timer(server: Arc<Server>, time_limit: Duration) {
    thread::spawn(move || loop {
        let role = &server.state.lock().unwrap().rule;
        match role {
            Rule::Candidater(_) => {
                if server.last_heart_beat_time.lock().unwrap().elapsed() > time_limit {
                    new_election(server.clone());
                    break;
                }
            }

            _ => {
                break;
            }
        }
    });
}

pub fn process_message(server: Arc<Server>, message_stream: TcpStream) {
    let message = serde_json::from_reader::<TcpStream, Message>(message_stream).unwrap();
    let mut state = server.state.lock().unwrap();
    match message {
        Message::HeartBeat(entry) => {
            let stream = TcpStream::connect(entry.leader_addr.clone()).unwrap();
            //不需要检查是不是leader节点 因为同一个term下，只有leader可能发心跳信息
            let term = &mut state.term;
            if entry.term >= *term {
                *term = entry.term;
                *server.last_heart_beat_time.lock().unwrap() = Instant::now();
                serde_json::to_writer(
                    stream,
                    &Message::AppendEntryReply(AppendEntryReply {
                        term: entry.term,
                        addr: "my addr".to_owned(),
                        apply_index: 0,
                        success: true,
                    }),
                )
                .unwrap();
            } else {
                serde_json::to_writer(
                    stream,
                    &Message::AppendEntryReply(AppendEntryReply {
                        term: entry.term,
                        addr: "my addr".to_owned(),
                        apply_index: 0,
                        success: false,
                    }),
                )
                .unwrap();
            }
        }

        Message::Log(entry) => {}

        Message::RequestVote(request_vote) => {
            let stream = TcpStream::connect(request_vote.addr.clone()).unwrap();
            let unvalid_log = (0, 0, AppendEntry::default());
            let last_log = state.log.last().unwrap_or(&unvalid_log);
            if state.term < request_vote.term
                && (last_log.0 < request_vote.cur_term
                    || (last_log.0 == request_vote.cur_term && last_log.1 < request_vote.cur_index))
            {
                state.vote_for = Some(request_vote.addr.clone());
                serde_json::to_writer(
                    &stream,
                    &Message::VoteFor(Vote {
                        term: state.term,
                        vote_granted: true,
                    }),
                )
                .unwrap();
            } else {
                serde_json::to_writer(
                    &stream,
                    &Message::VoteFor(Vote {
                        term: state.term,
                        vote_granted: true,
                    }),
                )
                .unwrap();
            }
        }

        Message::VoteFor(vote) => {
            if vote.term == state.term && vote.vote_granted == true {
                match &mut state.rule {
                    Rule::Candidater(count) => {
                        *count += 1;
                    }
                    _ => {}
                }
            } else if vote.term > state.term {
                state.rule = Rule::Follower;
                state.term = vote.term;
            }
        }

        _ => {}
    }
}
