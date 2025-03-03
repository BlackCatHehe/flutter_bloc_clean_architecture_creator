use crate::create_flutter_feature::template::{self};
use anyhow::Result;
use heck::ToSnakeCase;
use regex::Regex;
use std::{
    fs::{self, canonicalize},
    path::{Path, PathBuf},
};
pub struct Feature {
    name: String,
    path: PathBuf,
    use_injector: bool,
    as_router: bool,
}

fn validate_feature_path(path: &PathBuf) -> Result<PathBuf> {
    let path = canonicalize(path)?;
    if !path.exists() {
        return Err(anyhow::anyhow!("path not exists"));
    }
    println!("validate_feature_path: {:?}", path);
    if path.ends_with("features") {
        return Ok(path.to_path_buf());
    }
    let mut sub_dirs = fs::read_dir(&path)?;
    while let Some(sub_dir) = sub_dirs.next() {
        let dir_name = sub_dir.as_ref().unwrap().file_name();
        let is_dir = sub_dir.as_ref().unwrap().file_type()?.is_dir();
        if dir_name == "lib" && is_dir {
            return Ok(path.join("lib").join("features").to_path_buf());
        } else if dir_name == "features" && is_dir {
            return Ok(path.to_path_buf());
        } else {
            continue;
        }
    }
    Ok(path.to_path_buf())
}

impl Feature {
    pub fn new(path: &PathBuf, name: &str, use_injector: bool, as_router: bool) -> Self {
        Self {
            path: validate_feature_path(path).unwrap(),
            name: name.to_string(),
            use_injector,
            as_router,
        }
    }

    pub fn get_feature_path(&self) -> PathBuf {
        Path::new(&self.path).join(&self.name)
    }
}
impl Feature {
    fn validate_feature_name(&self) -> Result<()> {
        let regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
        if !regex.is_match(&self.name) {
            return Err(anyhow::anyhow!("feature name is invalid"));
        }
        Ok(())
    }
}

impl Feature {
    pub fn create(&mut self) -> Result<()> {
        self.validate_feature_name()?;
        // self.validate_feature_path()?;
        let feature_path = &self.get_feature_path();
        if feature_path.exists() {
            return Err(anyhow::anyhow!("feature {} already exists", self.name));
        }
        self.create_feature_folder(feature_path)?;
        self.create_bloc(feature_path)?;
        Ok(())
    }

    fn create_feature_folder(&self, path: &Path) -> Result<()> {
        println!(
            "============= create feature:{} folders =============",
            self.name
        );
        let folders = vec!["bloc", "pages", "widgets"];

        for folder in folders {
            let folder_path = &path.join(folder);
            std::fs::create_dir_all(folder_path)?;
        }
        Ok(())
    }

    fn create_bloc(&self, path: &Path) -> Result<()> {
        println!(
            "============= create feature:{} bloc =============",
            self.name
        );
        let bloc_path = &path.join("bloc");
        std::fs::create_dir_all(bloc_path)?;

        let snake_case_bloc_name = (&self.name).to_snake_case();
        let bloc_sub_folder_name = &bloc_path.join(&snake_case_bloc_name);
        std::fs::create_dir_all(bloc_sub_folder_name)?;

        let bloc_file_name = format!("{}_bloc.dart", snake_case_bloc_name);
        let bloc_file_path = bloc_sub_folder_name.join(bloc_file_name);
        let bloc_event_path =
            bloc_sub_folder_name.join(format!("{}_event.dart", snake_case_bloc_name));
        let bloc_state_path =
            bloc_sub_folder_name.join(format!("{}_state.dart", snake_case_bloc_name));

        std::fs::write(
            bloc_file_path,
            template::get_bloc_template(self.name.as_str(), self.use_injector),
        )?;
        std::fs::write(
            bloc_event_path,
            template::get_event_template(self.name.as_str()),
        )?;
        std::fs::write(
            bloc_state_path,
            template::get_state_template(self.name.as_str()),
        )?;

        let page_folder_path = &path.join("pages");
        let page_path = page_folder_path.join(format!("{}_page.dart", snake_case_bloc_name));
        std::fs::write(
            page_path,
            template::create_page_template(self.name.as_str(), self.as_router),
        )?;
        Ok(())
    }
}

#[test]
fn test_validate_feature_path() {
    use std::env::current_dir;
    let path = current_dir().unwrap().join("fucker");
    let feature_path = validate_feature_path(&path).unwrap();
    println!("feature_path: {:?}", feature_path);
}
