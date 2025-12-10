use anyhow::Result;

pub struct DeployResult {
    pub endpoint: String,
}

pub fn deploy(platform: &str, env: &str, _config: Option<&str>) -> Result<DeployResult> {
    println!("Deploying to {} (env: {})", platform, env);
    Ok(DeployResult {
        endpoint: format!("https://{}-{}.example.com", env, platform),
    })
}
