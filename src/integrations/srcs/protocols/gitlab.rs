use std::{
  path::PathBuf,
  process::{Command, Stdio},
  rc::Rc,
};

use crate::integrations::srcs::{DownloadProtocol, GitlabSource, ResolveRepoError, Source};

pub struct GlProtocol;

impl DownloadProtocol<GitlabSource> for GlProtocol {
  fn download(&self, source: Rc<GitlabSource>, path: PathBuf) -> Result<(), ResolveRepoError> {
    let command_maybe_error = Command::new("gh")
      .args([
        "repo",
        "clone",
        source.url().as_ref(),
        path
          .to_str()
          .ok_or(ResolveRepoError::SourceIdentifierInvalid)?,
      ])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output();

    let command = command_maybe_error.map_err(|_| ResolveRepoError::NotDownloadable)?;

    dbg!(command);

    Ok(())
  }
}
