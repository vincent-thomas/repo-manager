pub mod list;
pub mod protocols;
use serde::Deserialize;
use std::{
  option::Option,
  path::PathBuf,
  process::{Command, Stdio},
  rc::{self, Rc},
  result::Result,
  string::String,
  vec::Vec,
};
use thiserror::Error;
pub mod local;

use crate::config;

use self::list::Listable;

/// The source of the download, this can be shared across protocols and could be used by many.
pub trait Source: GenFullUrl {
  fn url(&self) -> SourceUrl;
}

#[derive(Clone)]
pub enum ListType {
  Local { path: PathBuf },
  Remote { source: Rc<dyn Source> },
}

#[derive(Clone)]
pub struct ListItem {
  pub ltype: ListType,
  pub name: String,
  pub icon: Option<char>,
}

impl ListItem {
  pub fn resolve_path<T: Source + Clone>(
    self,
    _prot: Rc<impl DownloadProtocol<T>>,
    path: PathBuf,
  ) -> PathBuf {
    match self.ltype {
      ListType::Local { path } => path,
      ListType::Remote { source: _source } => {
        // let _ = prot.download(*source, path.clone());
        path
      }
    }
  }
}

trait GenFullUrl {
  fn gen_url(&mut self, uri: String);
}

pub trait GitCompatSource: Source {}
pub trait TarCompatSource: Source {}

#[derive(Error, Debug)]
pub enum ResolveRepoError {
  #[error("Source identifier is invalid")]
  SourceIdentifierInvalid,
  #[error("Source is not downloadable")]
  NotDownloadable,
  #[error("AccessDenied trying to put source into folder")]
  FolderAccessDenied,
}

pub trait GitDownload: DownloadProtocol<dyn GitCompatSource> {}

pub trait DownloadProtocol<T>
where
  T: ?Sized + Source,
{
  fn download(&self, source: Rc<T>, path: PathBuf) -> Result<(), ResolveRepoError>;
}

#[derive(Clone)]
pub struct GithubSource {
  base_url: String,
  path: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RawRepoGhCli {
  name: String,
  url: String,
}

impl Listable for GithubSource {
  fn list_repos(&self, _: &config::Configuration) -> Result<Vec<ListItem>, ()> {
    let command = Command::new("gh")
      .args(["repo", "list", "--json", "name,url"])
      .stdout(Stdio::piped())
      .spawn()
      .unwrap()
      .wait_with_output();

    let test = command.unwrap().stdout;
    let string_nice = String::from_utf8(test).unwrap();
    let formatted: Vec<RawRepoGhCli> = serde_yaml::from_str(&string_nice).unwrap();

    Ok(
      formatted
        .iter()
        .map(|value| {
          let mut source = GithubSource::default();
          source.gen_url(value.url[18..].to_string());
          ListItem {
            name: value.name.clone(),
            icon: Option::Some(''),
            ltype: ListType::Remote {
              source: Rc::from(source),
            },
          }
        })
        .collect(),
    )
  }
}

struct SourceUrl(String);

impl AsRef<str> for SourceUrl {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Default for GithubSource {
  fn default() -> GithubSource {
    Self {
      path: None,
      base_url: "https://github.com".into(),
    }
  }
}

impl Source for GithubSource {
  fn url(&self) -> SourceUrl {
    SourceUrl(format!(
      "{}/{}",
      self.base_url,
      self.path.clone().expect("path is invalid")
    ))
  }
}

impl GenFullUrl for GithubSource {
  fn gen_url(&mut self, uri: String) {
    self.path = Some(uri);
  }
}

impl GitCompatSource for GithubSource {}

#[derive(Clone)]
pub struct GitlabSource {
  base_url: String,
  path: Option<String>,
}

impl Default for GitlabSource {
  fn default() -> GitlabSource {
    Self {
      path: None,
      base_url: "https://github.com".into(),
    }
  }
}
impl GenFullUrl for GitlabSource {
  fn gen_url(&mut self, uri: String) {
    self.path = Some(uri);
  }
}

impl Source for GitlabSource {
  fn url(&self) -> SourceUrl {
    let url = format!(
      "{}/{}",
      self.base_url,
      self.clone().path.clone().expect("Path not configured")
    );

    SourceUrl(url)
  }
}

impl GitCompatSource for GitlabSource {}

impl Listable for GitlabSource {
  fn list_repos(&self, _: &config::Configuration) -> Result<Vec<ListItem>, ()> {
    let result = Command::new("gh")
      .args(["repo", "list"])
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .stderr(Stdio::piped())
      .output();

    dbg!(result);

    Ok(todo!())
  }
}
