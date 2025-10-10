use std::process::Command;

pub struct Git;

impl Git {
    pub fn get_staged_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()?;

        if !output.status.success() {
            return Err("Failed to get staged files".into());
        }

        let files = String::from_utf8(output.stdout)?
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        Ok(files)
    }

    pub fn get_staged_diff(files: &[String]) -> Result<String, Box<dyn std::error::Error>> {
        let mut command = Command::new("git");
        command.args(["diff", "--cached"]);

        if !files.is_empty() {
            command.arg("--");
            for file in files {
                command.arg(file);
            }
        }

        let output = command.output()?;

        if !output.status.success() {
            return Err("Failed to get staged diff".into());
        }

        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn commit(message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("git")
            .args(["commit", "-m", message])
            .output()?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to commit: {error}").into());
        }

        Ok(())
    }
}
