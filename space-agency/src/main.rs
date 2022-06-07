
use personnel::{Candidate, AstronautJob};

fn get_job_code(job : &AstronautJob) ->u16 {
    match job{
        Biogeochemist => 251,
        Biologist => 257,
        Engineer => 263,
        Geologist => 269,
        Mechanic => 271,
        Medic => 277,
        RoverOp => 281,
        Scientist => 283,
    }
}

fn get_job_score(candid: &Candidate) -> u16{
    let primary = get_job_code(&candid.primary_job);
    let secondary = match &candid.secondary_job{
        None => 0,
        Some(job) => get_job_code(&job),
    };
    

    (primary + secondary) % 576
}

fn get_candid_score(candid: &Candidate) ->u16{
    let score = candid.health as u16 + get_job_score(&candid) + candid.age as u16;
    score % 3928
}

fn main() {
  let mut candid_list: Vec<Candidate> = Candidate::load_candidate_file();
  candid_list.sort_by(|a,b|{
        get_job_score(b).cmp(&get_job_score(a))
  });
}

