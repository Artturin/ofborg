/// Build attributes within a Nixpkgs PR the way that ofBorg does
use ofborg;
use ofborg::nix::Nix;
use ofborg::notifyworker::SimpleNotifyWorker;
use ofborg::tasks::build::BuildWorker;
use ofborg::tasks::evaluate::EvaluationWorker;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ofborg::setup_log();
    let current_system = "x86_64-linux";

    let attrs = vec!["tmux".to_owned()];

    let nix = Nix::new(
        current_system.to_owned(),
        "daemon".to_owned(),
        90 * 60,
        None,
    );
    let p: std::path::PathBuf = "/tmp/ofborg".into();
    let cloner = ofborg::checkout::cached_cloner(&p);
    let worker = EvaluationWorker::new(
        cloner,
        nix,
        current_system.to_owned(),
        "one-off".to_string(),
    );
    let repo = ofborg::message::Repo {
        owner: "nixos".to_owned(),
        name: "nixpkgs".to_owned(),
        full_name: "NixOS/nixpkgs".to_owned(),
        clone_url: "https://github.com/nixos/nixpkgs.git".to_owned(),
    };

    let mut dummy_receiver = ofborg::notifyworker::DummyNotificationReceiver::new();

    let pr = ofborg::message::Pr {
        target_branch: Some("master".to_owned()),
        number: 201183,
        head_sha: "2af809015a65810571e7e8d8541b4ca7ba25b8d4".to_owned(),
    };

    let evaljob = ofborg::message::evaluationjob::EvaluationJob { repo, pr };

    let job = ofborg::tasks::eval::NixpkgsStrategy::new(job, pull, issue, issue_ref, repo, gists, nix)

    //let job = ofborg::message::buildjob::BuildJob::new(
    //    repo,
    //    pr,
    //    ofborg::commentparser::Subset::Nixpkgs,
    //    /* attrs */ attrs,
    //    /* logs */ None,
    //    /* statusreport: */ None,
    //    /*request_id: */ "one-off".to_string(),
    //);

    worker.consumer(&job, &mut dummy_receiver);

    for message in dummy_receiver.actions {
        use ofborg::worker::Action;
        match message {
            Action::Ack | Action::NackRequeue | Action::NackDump => println!("{:?}", message),
            Action::Publish(msg) => match msg.content_type {
                Some(x) if x == "application/json" => {
                    let data: serde_json::Value = serde_json::from_slice(&msg.content).unwrap();
                    println!("Action::Publish: {:?}", data);
                }
                _ => {
                    println!("Action::Publish: {:?}", msg);
                }
            },
        }
    }

    Ok(())
}
