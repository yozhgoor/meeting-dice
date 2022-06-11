use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub last_chair: Option<String>,
    pub last_note_taker: Option<String>,
    pub members: Vec<String>,
}

impl Data {
    fn new() -> Self {
        Self {
            last_chair: None,
            last_note_taker: None,
            members: Vec::new(),
        }
    }

    pub fn get_or_create() -> Result<Self> {
        let data_file_path = get_data_file_path()?;

        let data: Self = match fs::read(&data_file_path) {
            Ok(file) => serde_json::from_slice(&file)?,
            Err(_) => {
                let data = Self::new();
                fs::write(&data_file_path, serde_json::to_string(&data)?)?;
                data
            }
        };

        Ok(data)
    }

    pub fn get_member_id(&self, name: impl AsRef<str>) -> Option<usize> {
        if self.members.is_empty() {
            println!("The members list is empty");
            return None;
        }

        let mut ids = self
            .members
            .iter()
            .enumerate()
            .filter_map(|(id, member)| {
                if member.to_lowercase() == name.as_ref().to_lowercase() {
                    Some(id)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if ids.len() == 1 {
            ids.pop()
        } else {
            println!("Error: More than one id for {}", name.as_ref());
            None
        }
    }

    pub fn change_last_chair(&mut self, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        if let Some(_id) = self.get_member_id(name) {
            self.last_chair = Some(name.to_string());
        } else {
            bail!("{} doesn't exists in the members list", name);
        }

        Ok(())
    }

    pub fn change_last_note_taker(&mut self, name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        if let Some(_id) = self.get_member_id(name) {
            self.last_note_taker = Some(name.to_string());
        } else {
            bail!("{} doesn't exists in the members list", name);
        }

        Ok(())
    }

    pub fn add_members(&mut self, names: Vec<impl AsRef<str>>) {
        for name in names {
            if let Some(_id) = self.get_member_id(name.as_ref()) {
                println!("{} already exists in the members list", name.as_ref())
            } else {
                self.members.push(name.as_ref().to_string())
            }
        }
    }

    pub fn remove_members(&mut self, names: Vec<impl AsRef<str>>) {
        for name in names {
            if let Some(id) = self.get_member_id(name.as_ref()) {
                if let Some(last_chair) = &self.last_chair {
                    if last_chair == name.as_ref() {
                        self.last_chair = None;
                    }
                }
                if let Some(last_note_taker) = &self.last_note_taker {
                    if last_note_taker == name.as_ref() {
                        self.last_note_taker = None;
                    }
                }
                self.members.remove(id);
            } else {
                println!("{} doesn't exists in the members list", name.as_ref())
            }
        }
    }

    pub fn save(self) -> Result<()> {
        let data_file_path = get_data_file_path()?;
        fs::write(&data_file_path, serde_json::to_string(&self)?)?;

        println!("Saved! Have a great meeting");
        Ok(())
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

use std::path::PathBuf;

fn get_data_file_path() -> Result<PathBuf> {
    #[cfg(unix)]
    let data_file_path = {
        let data_dir = xdg::BaseDirectories::with_prefix("meeting-dice")?;
        data_dir.place_data_file("data.json")?
    };

    #[cfg(windows)]
    let data_file_path = {
        let data_dir = dirs::data_dir()
            .expect("cannot get data directory")
            .join(env!("CARGO_PKG_NAME"));
        let _ = fs::create_dir_all(&data_dir);

        data_dir.join("data.json")
    };

    Ok(data_file_path)
}
