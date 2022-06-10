use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub last_chair: Option<String>,
    pub members: Vec<String>,
}

impl Data {
    fn new() -> Self {
        Self {
            last_chair: None,
            members: Vec::new(),
        }
    }

    pub fn get_or_create() -> Result<Self> {
        #[cfg(unix)]
        let data_file_path = {
            let data_dir = xdg::BaseDirectories::with_prefix("chair-chooser")?;
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

    pub fn add_member(&mut self, name: impl AsRef<str>) {
        let name = name.as_ref();
        if self.get_member_id(name).is_none() {
            println!("{} already exists in the members list", name)
        } else {
            self.members.push(name.to_string())
        }
    }

    pub fn add_members(&mut self, names: Vec<impl AsRef<str>>) {
        for name in names {
            self.add_member(name)
        }
    }

    pub fn remove_member(&mut self, name: impl AsRef<str>) {
        if let Some(id) = self.get_member_id(name.as_ref()) {
            self.members.remove(id);
        } else {
            println!("{} doesn't exists in the members list", name.as_ref())
        }
    }

    pub fn remove_members(&mut self, names: Vec<impl AsRef<str>>) {
        for name in names {
            self.remove_member(name)
        }
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Common

    fn empty_team() -> Data {
        Data::new()
    }

    fn one_member_team_without_last_chair() -> Data {
        Data {
            last_chair: None,
            members: vec!["John".to_string()],
        }
    }

    fn one_member_team_with_last_chair() -> Data {
        Data {
            last_chair: Some("John".to_string()),
            members: vec!["John".to_string()],
        }
    }

    fn three_members_team_without_last_chair() -> Data {
        Data {
            last_chair: None,
            members: vec!["John".to_string(), "Sonia".to_string(), "Tommy".to_string()],
        }
    }

    fn three_members_team_with_last_chair() -> Data {
        Data {
            last_chair: Some("Sonia".to_string()),
            members: vec!["John".to_string(), "Sonia".to_string(), "Tommy".to_string()],
        }
    }

    // Tests

    #[test]
    fn change_last_chair_on_empty_team() {
        let mut data = empty_team();

        let res = data.change_last_chair("John");
        assert!(res.is_err())
    }

    #[test]
    fn change_last_chair_on_one_member_team_without_last_chair() -> Result<()> {
        let mut data = one_member_team_without_last_chair();

        data.change_last_chair("John")?;
        assert_eq!(data.last_chair, Some("John".to_string()));

        let res = data.change_last_chair("Sonia");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn change_last_chair_on_one_member_team_with_last_chair() -> Result<()> {
        let mut data = one_member_team_with_last_chair();

        data.change_last_chair("John")?;
        assert_eq!(data.last_chair, Some("John".to_string()));

        let res = data.change_last_chair("Sonia");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn change_last_chair_on_three_member_team_without_last_chair() -> Result<()> {
        let mut data = three_members_team_without_last_chair();

        data.change_last_chair("John")?;
        assert_eq!(data.last_chair, Some("John".to_string()));
        data.change_last_chair("Sonia")?;
        assert_eq!(data.last_chair, Some("Sonia".to_string()));
        data.change_last_chair("Tommy")?;
        assert_eq!(data.last_chair, Some("Tommy".to_string()));

        let res = data.change_last_chair("Noah");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn change_last_chair_on_three_member_team_with_last_chair() -> Result<()> {
        let mut data = three_members_team_with_last_chair();

        data.change_last_chair("John")?;
        assert_eq!(data.last_chair, Some("John".to_string()));
        data.change_last_chair("Sonia")?;
        assert_eq!(data.last_chair, Some("Sonia".to_string()));
        data.change_last_chair("Tommy")?;
        assert_eq!(data.last_chair, Some("Tommy".to_string()));

        let res = data.change_last_chair("Noah");
        assert!(res.is_err());

        Ok(())
    }

    #[test]
    fn get_member_id_on_empty_team() {
        let data = empty_team();

        assert_eq!(data.get_member_id("John"), None);
    }

    #[test]
    fn get_member_id_on_empty_team_lowercase() {
        let data = empty_team();

        assert_eq!(data.get_member_id("john"), None);
    }

    #[test]
    fn get_member_id_on_one_member_team_without_last_chair() {
        let data = one_member_team_without_last_chair();

        assert_eq!(data.get_member_id("John"), Some(0));
        assert_eq!(data.get_member_id("Sonia"), None);
    }

    #[test]
    fn get_member_id_on_one_member_team_without_last_chair_lowercase() {
        let data = one_member_team_without_last_chair();

        assert_eq!(data.get_member_id("john"), Some(0));
        assert_eq!(data.get_member_id("sonia"), None);
    }

    #[test]
    fn get_member_id_on_one_member_team_with_last_chair() {
        let data = one_member_team_with_last_chair();

        assert_eq!(data.get_member_id("John"), Some(0));
        assert_eq!(data.get_member_id("Sonia"), None);
    }

    #[test]
    fn get_member_id_on_one_member_team_with_last_chair_lowercase() {
        let data = one_member_team_with_last_chair();

        assert_eq!(data.get_member_id("john"), Some(0));
        assert_eq!(data.get_member_id("sonia"), None);
    }

    #[test]
    fn get_member_id_on_three_member_team_without_last_chair() {
        let data = three_members_team_without_last_chair();

        assert_eq!(data.get_member_id("John"), Some(0));
        assert_eq!(data.get_member_id("Sonia"), Some(1));
        assert_eq!(data.get_member_id("Tommy"), Some(2));
        assert_eq!(data.get_member_id("Noah"), None);
    }

    #[test]
    fn get_member_id_on_three_member_team_without_last_chair_lowercase() {
        let data = three_members_team_without_last_chair();

        assert_eq!(data.get_member_id("john"), Some(0));
        assert_eq!(data.get_member_id("sonia"), Some(1));
        assert_eq!(data.get_member_id("tommy"), Some(2));
        assert_eq!(data.get_member_id("noah"), None);
    }

    #[test]
    fn get_member_id_on_three_member_team_with_last_chair() {
        let data = three_members_team_with_last_chair();

        assert_eq!(data.get_member_id("John"), Some(0));
        assert_eq!(data.get_member_id("Sonia"), Some(1));
        assert_eq!(data.get_member_id("Tommy"), Some(2));
        assert_eq!(data.get_member_id("Noah"), None);
    }

    #[test]
    fn get_member_id_on_three_member_team_with_last_chair_lowercase() {
        let data = three_members_team_with_last_chair();

        assert_eq!(data.get_member_id("john"), Some(0));
        assert_eq!(data.get_member_id("sonia"), Some(1));
        assert_eq!(data.get_member_id("tommy"), Some(2));
        assert_eq!(data.get_member_id("noah"), None);
    }
}
