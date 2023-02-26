pub mod data {

    // An instantiation of an audit. Contains information about the set of ballots
    // under audit, as well as the contests that statistically drive the audit
    // (targeted) and contests which have statistics computed for them but that do
    // not impact the status of the audit (opportunistic)
    struct Audit {
        Name: String, // name for the audit
        Ballots: Vec<Ballot>,
        Targeted: Vec<Contest>,
        Opportunistic: Vec<Contest>,
    }

    // Contest information for the audit. Includes the contest name, how many
    // winners the contest has, who won and who lost, and what the social choice
    // function was for the contest.
    struct Contest {
        Name: String,
        NumWinners: i32, // number of winners
        Winners: Vec<Strings>, // list of winners
        Losers: Vec<Strings>, // list of losers (note, winners plus losers 
                              // = all candidates
        SocialChoice: ChoiceFunction // what social choice function was used to
                                     // tabulate the election
    }

    enum ChoiceFunction {
        Plurality, // AKA First-past-the-post
        Majority,
        SuperMajority{threshold: i32},
        InstantRunoff,
        SingleTranferrable,
    }

    // An individual ballot and all of the votes it contains.
    struct Ballot {
        Contests: Vec<Vote>
    }

    // Votes for a contest that appear on a ballot. Includes the name of the 
    // contest, the names of candidates for that contest, and the marks made for 
    // that contest.
    struct Vote {
        Name: String,
        Candidates: Vec<String>,
        Marks: Vec<Mark>,
    }

    // What marks are on the ballot. See MarkType. 
    struct Mark {
        Type: MarkType,
        Value: i32,
    }

    // Types of possible marks that can appear on the ballot. Include Ranks, which
    // pertain to ranked-choice voting systems, and selections, which are boolean
    // selections for non-ranked contests. 
    enum MarkType {
        Rank,
        Selection,
    }
}
