use std::{
    env::{self, set_current_dir},
    path::{Path, PathBuf},
};

use anyhow::Result;
use clap::{Parser, Subcommand};
use create_flutter_feature::Feature;
use create_flutter_project::Project;
use exec_cmd::exec;

mod create_flutter_feature;
mod create_flutter_project;
mod exec_cmd;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    CreateFeature {
        #[arg(short, long)]
        name: String,

        #[arg(short, long, value_name = "path", value_hint = clap::ValueHint::DirPath)]
        feature_path: PathBuf,

        #[arg(short, long, default_value = "true")]
        use_injector: bool,

        #[arg(short, long, default_value = "true")]
        as_router: bool,
    },
    CreateProject {
        #[arg(short, long)]
        name: String,

        #[arg(short, long)]
        org: String,

        #[arg(short, long, default_value = ".")]
        project_path: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("{:?}", args);

    match args.command {
        Command::CreateFeature {
            name,
            feature_path,
            use_injector,
            as_router,
        } => {
            let mut feature = Feature::new(&feature_path, &name, use_injector, as_router);
            feature.create()?;
        }
        Command::CreateProject {
            name,
            org,
            project_path,
        } => {
            let path = if project_path == "." {
                env::current_dir().unwrap().to_str().unwrap().to_string()
            } else {
                project_path
            };

            if !Path::new(&path).exists() {
                return Err(anyhow::anyhow!("path not exists"));
            }

            let project = Project::new(&path, &name, &org);
            project.create()?;

            let mut feature = Feature::new(
                &PathBuf::from(&path)
                    .join(&name)
                    .join("lib")
                    .join("features"),
                "home",
                true,
                true,
            );
            feature.create()?;

            set_current_dir(PathBuf::from(&path).join(&name))?;
            exec("dart", &["run", "build_runner", "build"])?;
        }
    }
    Ok(())
}
