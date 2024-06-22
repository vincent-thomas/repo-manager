use std::{
  path::PathBuf,
  process::{Command, Stdio},
};

use crate::integrations::srcs::{DownloadProtocol, GitCompatSource, ResolveRepoError};

pub struct GitProtocol;

impl<T> DownloadProtocol<T> for GitProtocol
where
  T: GitCompatSource,
{
  fn download(&self, source: &T, path: PathBuf) -> Result<(), ResolveRepoError> {
    let command_maybe_error = Command::new("git")
      .args([
        "clone",
        source.url().as_ref(),
        path.as_os_str().to_str().expect("non utf-8 path"),
      ])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output();

    // TODO: Den ska kanske användas?
    let _ = command_maybe_error.map_err(|_| ResolveRepoError::NotDownloadable)?;

    Ok(())
  }
}
