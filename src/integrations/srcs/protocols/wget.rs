use crate::integrations::srcs::{DownloadProtocol, ResolveRepoError, Source, TarCompatSource};
use std::{process::Command, rc::Rc};

// Define a Wget protocol struct
pub struct Wget;

// Implement the DownloadProtocol trait for Wget, restricted to HttpSource
impl<S: TarCompatSource> DownloadProtocol<S> for Wget {
  fn download(&self, source: Rc<S>, path: std::path::PathBuf) -> Result<(), ResolveRepoError> {
    // TODO: Kanske fixa så att detta inte är inte använt
    let _ = Command::new("wget").args([
      source.url().as_ref(),
      "-P",
      path.as_os_str().to_str().expect("Not valid utf-8 string"),
    ]);

    todo!()
  }
}
