use std::error::Error;
use std::fs;
use std::fs::{DirBuilder, File};
use std::io::Write;
use std::path::PathBuf;
use crate::project::config_file::{Build, Config, Package, Target};
use crate::ProjectType;

enum ProjectItem<'a> {
    File(ProjectFile<'a>),
    Directory(ProjectDirectory<'a>),
}

struct ProjectFile<'a> {
    directory: &'a str,
    name: &'a str,
    extension: &'a str,
    content: &'a str,
}

impl ProjectFile<'_> {
    pub fn create_file(&self, base_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let directory_path = PathBuf::from(base_path).join(&self.directory);

        DirBuilder::new().recursive(true).create(&directory_path)?;

        let file_path = directory_path.join(self.name.to_owned() + &*self.extension);

        let mut file = File::create_new(file_path)?;

        file.write_all(self.content.as_bytes())?;
        file.flush()?;

        Ok(())
    }
}

struct ProjectDirectory<'a> {
    directory: &'a str,
}

impl ProjectDirectory<'_> {
    pub fn create_directory(&self, base_path: &PathBuf) -> Result<(), Box<dyn Error>> {
        let directory_path = PathBuf::from(base_path).join(&self.directory);

        DirBuilder::new().recursive(true).create(directory_path)?;

        Ok(())
    }
}

struct ProjectStructure<'a> {
    base_path: PathBuf,
    items: Vec<ProjectItem<'a>>,
}

impl ProjectStructure<'_> {
    pub fn create(&self) -> Result<(), Box<dyn Error>> {
        for item in &self.items {
            match item {
                ProjectItem::File(file) => file.create_file(&self.base_path)?,
                ProjectItem::Directory(directory) => directory.create_directory(&self.base_path)?,
            }
        }

        Ok(())
    }
}

pub fn init(name: String, project_type: ProjectType) -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from("./");

    if name != "." {
        if fs::exists("./".to_owned() + name.as_str())? {
            return Err(format!("Directory '{}' already exists!", name).into());
        }

        path.push(&name);

        fs::create_dir(&path)?;
    }
    
    let config = Config {
        package: Package {
            name,
            version: "0.1.0".to_string(),
            r#type: project_type.clone()
        },
        build: Build {
            target: Target::Linux
        },
    };
    
    let config_file_content = toml::to_string(&config)?;

    let project_structure = ProjectStructure {
        base_path: path,
        items: vec![
            ProjectItem::Directory(ProjectDirectory {
                directory: "src",
            }),
            ProjectItem::Directory(ProjectDirectory {
                directory: "packages",
            }),
            ProjectItem::Directory(ProjectDirectory {
                directory: "docs",
            }),
            ProjectItem::Directory(ProjectDirectory {
                directory: "tests",
            }),
            ProjectItem::Directory(ProjectDirectory {
                directory: "build",
            }),

            ProjectItem::File(ProjectFile {
                directory: ".",
                name: "axiom",
                extension: ".toml",
                content: &*config_file_content
            }),
            ProjectItem::File(ProjectFile {
                directory: ".",
                name: "axiom",
                extension: ".lock",
                content: ""
            }),
            ProjectItem::File(ProjectFile {
                directory: "src",
                name: match project_type {
                    ProjectType::Bin => "bin",
                    ProjectType::Lib => "lib",
                },
                extension: ".axiom",
                content: ""
            }),
        ]
    };

    project_structure.create()?;

    Ok(())
}