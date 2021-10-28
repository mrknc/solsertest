use solana_vote_program::vote_state::{VoteState, VoteStateVersions, Lockout, VoteInit};
use solana_sdk::{pubkey::Pubkey, clock::Clock};
use std::{str::FromStr};
use std::fmt::Write;

// adapted from solana/programs/vote/src/vote_state/mod/rs#test_vote_serialize()
// requires changes to programs/vote/src/lib.rs to export all the structs
fn main() {
    let npk = Pubkey::from_str("94P9uVCyfoX5ayPQjUo5XQKqRVyfN9BbznJqew453xSr").unwrap();
    let wpk = Pubkey::from_str("A7FbaDow3bqMAiCLbdtbfXv6hDB1oTrNbHU1U8Ndkf86").unwrap();

    let mut buffer: Vec<u8> = vec![0; VoteState::size_of()];
    let mut vote_state = VoteState::new(
        &VoteInit {
            node_pubkey: npk,
            authorized_voter: npk,
            authorized_withdrawer: wpk,
            commission: 7
        }, &Clock::default());
    vote_state
        .votes
        .resize(31, Lockout::default()); //MAX_LOCKOUT_HISTORY
    let versioned = VoteStateVersions::new_current(vote_state);
    VoteState::serialize(&versioned, &mut buffer).unwrap();
    // TODO there must be a better way
    let mut s = String::with_capacity(2 * buffer.len());
    for byte in &buffer {
        write!(s, "{:02x}", byte);
    }
    println!("{}", s);
    assert_eq!(
        VoteStateVersions::new_current(VoteState::deserialize(&buffer).unwrap()),
        versioned
    );
}
