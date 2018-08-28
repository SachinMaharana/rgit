#[derive(Clone)]
pub struct Config {
    pub repo_name: String,
    pub repo_desc: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not Enough Arguments. Provide Repo Name.");
        }
        let repo_name = args[1].clone();
        let repo_desc = args[2].clone();

        Ok(Config {
            repo_name,
            repo_desc,
        })
    }
}
