use std::{collections::HashMap, env::set_current_dir, path::Path};

use crate::exec_cmd::exec;
use anyhow::Result;

use super::template;

#[derive(Debug)]
pub struct Project {
    /// project create at is path
    path: String,
    /// project name
    name: String,
    /// project org
    org: String,
}

impl Project {
    pub fn new(path: &str, name: &str, org: &str) -> Self {
        Self {
            path: path.to_string(),
            name: name.to_string(),
            org: org.to_string(),
        }
    }
}
impl Project {
    pub fn get_project_root_path(&self) -> String {
        Path::new(&self.path)
            .join(&self.name)
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn valid_project_path(&self) -> bool {
        Path::new(&self.path).join(&self.name).exists()
    }
}

impl Project {
    pub fn create(&self) -> Result<()> {
        println!("start create project at: {}, org: {}", self.path, self.org);

        if self.valid_project_path() {
            return Err(anyhow::anyhow!("project already exists"));
        }
        set_current_dir(&self.path).expect("set create directory error");
        exec("flutter", &["create", &self.name, "--org", &self.org, "-e"])?;
        self.update_project_config()?;
        Ok(())
    }
}

impl Project {
    pub fn update_project_config(&self) -> Result<()> {
        let project_root_path = self.get_project_root_path();
        let path = Path::new(&project_root_path);
        set_current_dir(&project_root_path).expect("set project root directory error");
        self.update_lints(path).expect("update lints error");
        self.add_dependencies().expect("add dependencies error");
        self.create_clean_folders(&path)
            .expect("create clean folders error");
        self.config_core_template(&path)
            .expect("config core template error");
        self.config_main_template(&path)
            .expect("config main template error");
        self.config_easy_localization(&path)
            .expect("config easy localization error");
        Ok(())
    }

    pub fn update_lints(&self, path: &Path) -> Result<()> {
        let lints_template = template::lints_template();
        let file_path = path.join("analysis_options.yaml");
        std::fs::write(file_path, lints_template)?;
        Ok(())
    }

    fn add_dependencies(&self) -> Result<()> {
        println!("============= add dependencies =============");
        let dependencies = vec![
            // freezed
            "freezed_annotation",
            "dev:freezed",
            // json
            "json_annotation",
            "dev:json_serializable",
            // bloc
            "flutter_bloc",
            // auto_route
            "auto_route",
            "dev:auto_route_generator",
            // get_it
            "get_it",
            "injectable",
            "dev:injectable_generator",
            // flex_color_scheme
            "flex_color_scheme",
            // easy_localization
            "easy_localization",
        ];

        exec("flutter", &[vec!["pub", "add"], dependencies].concat())?;
        Ok(())
    }

    fn create_clean_folders(&self, path: &Path) -> Result<()> {
        println!("============= create clean folders =============");
        let folders = HashMap::from([
            (
                "core",
                vec![
                    "router",
                    "injection",
                    "constants",
                    "error",
                    "network",
                    "secrets",
                    "theme",
                    "usecase",
                    "utils",
                ],
            ),
            ("features", vec![]),
        ]);
        for (folder, sub_folders) in folders {
            let folder_path = &path.join("lib").join(folder);
            std::fs::create_dir_all(folder_path)?;
            for sub_folder in sub_folders {
                let sub_folder_path = folder_path.join(sub_folder);
                std::fs::create_dir_all(sub_folder_path)?;
            }
        }
        Ok(())
    }

    fn config_core_template(&self, path: &Path) -> Result<()> {
        println!("============= config core template =============");
        let core_path = &path.join("lib").join("core");
        std::fs::write(
            core_path.join("theme").join("app_theme.dart"),
            template::create_app_theme_file(),
        )?;
        std::fs::write(
            core_path.join("injection").join("injector.dart"),
            template::create_injector_file(),
        )?;
        std::fs::write(
            core_path.join("router").join("app_router.dart"),
            template::create_router_file(),
        )?;
        Ok(())
    }
    fn config_easy_localization(&self, path: &Path) -> Result<()> {
        println!("============= config easy localization =============");
        let pubspec_path = &path.join("pubspec.yaml");
        let mut pubspec_content = std::fs::read_to_string(pubspec_path)?;

        pubspec_content = pubspec_content.replace(
            "uses-material-design: true",
            r#"uses-material-design: true
  assets:
    - assets/translations/
"#,
        );
        std::fs::write(pubspec_path, pubspec_content)?;

        let translations_path = &path.join("assets").join("translations");
        std::fs::create_dir_all(translations_path)?;
        std::fs::write(
            translations_path.join("en-US.json"),
            r#"{
    "example": {
        "hello": "Hello",
        "world": "World!",
        "helloWorld": "@:example.hello @:example.world",
        "date": "Today is {dateTime}"
    }
}"#,
        )?;
        std::fs::write(
            translations_path.join("zh-CN.json"),
            r#"{
    "example": {
        "hello": "你好",
        "world": "世界！",
        "helloWorld": "@:example.hello @:example.world",
        "date": "今天是 {dateTime}"
    }
}"#,
        )?;

        self.config_ios_info_plist(path)?;

        Ok(())
    }

    fn config_ios_info_plist(&self, path: &Path) -> Result<()> {
        println!("============= config ios info plist =============");
        let file_path = &path.join("ios").join("Runner").join("Info.plist");
        let mut value = plist::Value::from_file(file_path)?;
        if let plist::Value::Dictionary(ref mut dict) = value {
            dict.insert(
                "CFBundleLocalizations".to_string(),
                plist::Value::Array(vec![
                    plist::Value::String("en".to_string()),
                    plist::Value::String("zh".to_string()),
                ]),
            );
        };
        value.to_file_xml(file_path)?;
        Ok(())
    }

    fn config_main_template(&self, path: &Path) -> Result<()> {
        println!("============= config main template =============");
        let main_path = path.join("lib").join("main.dart");
        std::fs::write(main_path, template::create_main_file())?;
        Ok(())
    }
}

impl Project {}

#[test]
fn test_config_easy_localizationte() {
    use std::env;
    let project_path = &env::current_dir().unwrap().join("fucker");
    println!("project_path: {}", project_path.display());
    Project::new("", "fucker", "fucker")
        .config_core_template(project_path)
        .unwrap();
}
