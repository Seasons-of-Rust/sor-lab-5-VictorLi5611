#[cfg(test)]
fn main(){
    let a = Candidate{
        primary_job: AstronautJob::Biogeochemist,
        secondary_job: None,
        age:25,
        health:72
    };

    assert_eq!(calculate_job_score(&a,217))
}