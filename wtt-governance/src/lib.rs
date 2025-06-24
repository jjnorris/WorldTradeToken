//! Simple governance module with proposal and voting logic.
//! This is a minimal placeholder implementation and not production ready.

#[derive(Debug, PartialEq, Eq)]
pub enum ProposalState {
    Pending,
    Active,
    Passed,
    Rejected,
}

/// A governance proposal.
pub struct Proposal {
    pub id: u64,
    pub description: String,
    pub start: u64,
    pub duration: u64,
    pub votes_for: u64,
    pub votes_against: u64,
}

impl Proposal {
    /// Create a new proposal that starts at `start` and lasts `duration` seconds.
    pub fn new(id: u64, description: impl Into<String>, start: u64, duration: u64) -> Self {
        Self {
            id,
            description: description.into(),
            start,
            duration,
            votes_for: 0,
            votes_against: 0,
        }
    }

    /// Cast a vote. `support` indicates a yes or no vote. `amount` is the weight.
    pub fn vote(&mut self, support: bool, amount: u64) {
        if support {
            self.votes_for += amount;
        } else {
            self.votes_against += amount;
        }
    }

    /// Determine the current state given `now` and required `quorum`.
    pub fn state(&self, now: u64, quorum: u64) -> ProposalState {
        if now < self.start {
            ProposalState::Pending
        } else if now < self.start + self.duration {
            ProposalState::Active
        } else {
            let total_votes = self.votes_for + self.votes_against;
            if total_votes < quorum {
                ProposalState::Rejected
            } else if self.votes_for > self.votes_against {
                ProposalState::Passed
            } else {
                ProposalState::Rejected
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_lifecycle() {
        let mut p = Proposal::new(1, "Test", 10, 10);
        assert_eq!(p.state(0, 1), ProposalState::Pending);
        assert_eq!(p.state(10, 1), ProposalState::Active);
        p.vote(true, 5);
        p.vote(false, 3);
        assert_eq!(p.state(15, 1), ProposalState::Active);
        // after end with quorum met and more yes votes
        assert_eq!(p.state(21, 8), ProposalState::Passed);
        // without quorum it fails
        assert_eq!(p.state(21, 20), ProposalState::Rejected);
    }
}
