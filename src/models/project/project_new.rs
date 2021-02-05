use crate::DbConn;
use diesel::prelude::*;
use crate::db;
use crate::models::Project;
use crate::vcs::VCS;
use crate::db::schema::*;
use std::path::Path;
use anyhow::{Context, Result};

pub struct NewProject {
    pub user_id: i32,
    pub ownername: String,
    pub name: String,
    pub description: Option<String>,
    pub website: Option<String>,
    pub default_branch: Option<String>,

    pub is_private: bool,
    pub is_empty: bool,

    pub is_fork: bool,
    pub forked_project: Option<i32>,
    pub disk_size: usize,
    pub vcs: u32,
}

impl NewProject {
    pub fn from_form(conn: &DbConn, project_form: &NewProjectForm) -> Result<Project> {
        let init = project_form.init;
        let user = db::User::by_id(conn, project_form.owner_id)
            .context("Could not find User for Owner in FromForm for NewProject.")?;
        let project = NewProject {
                user_id: project_form.owner_id,
                ownername: user.username.clone(),
                name: project_form.name.clone(),
                description: project_form.description.clone(),
                website: None,
                default_branch: None,
                is_private: project_form.is_private,
                is_empty: !init,
                is_fork: bool::default(),
                forked_project: None,
                disk_size: usize::default(),
                vcs: project_form.vcs as u32,
            };
        
        let project = project.create_and_insert(conn, &user.get_projects_dir())
                .context("Could not create a project in FromForm for NewProject.")?;
        if init {
            project.init(conn)
                .context("Could not init VCS of project in FromForm for NewProject.")?;
        }
        Ok(project)
    }

    pub fn create_and_insert(&self, conn: &DbConn, location: &Path) -> Result<Project> {
        let _vcs = VCS::create(self.vcs, location, &self.name)?;
        db::Project::new(conn, self)
            .context("Could not init VCS of project in FromForm for NewProject.")
    }
}

impl<'a> Insertable<projects::table> for &'a NewProject {
    #![allow(clippy::type_complexity)]
    type Values = <(
        diesel::dsl::Eq<projects::user_id, i32>,
        diesel::dsl::Eq<projects::name, &'a String>,
        Option<diesel::dsl::Eq<projects::description, &'a String>>,Option<diesel::dsl::Eq<projects::website, &'a String>>,
        Option<diesel::dsl::Eq<projects::default_branch, &'a String>>,
        diesel::dsl::Eq<projects::is_private, bool>,diesel::dsl::Eq<projects::is_empty, bool>,
        diesel::dsl::Eq<projects::is_fork, bool>,Option<diesel::dsl::Eq<projects::forked_project, i32>>,
        diesel::dsl::Eq<projects::disk_size, i64>,
    ) as Insertable<projects::table>>::Values;

    fn values(self) -> Self::Values {
        (
            projects::user_id.eq(self.user_id),
            projects::name.eq(&self.name),
            self.description.as_ref().map(|s| projects::description.eq(s)),
            self.website.as_ref().map(|s| projects::website.eq(s)),
            self.default_branch.as_ref().map(|s| projects::default_branch.eq(s)),

            projects::is_private.eq(self.is_private),
            projects::is_empty.eq(self.is_empty),

            projects::is_fork.eq(self.is_fork),
            self.forked_project.map(|s| projects::forked_project.eq(s)),
            projects::disk_size.eq(self.disk_size as i64),
        ).values()
    }
}

#[derive(FromForm)]
pub struct NewProjectForm {
    pub owner_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub init: bool,
    pub vcs: i32,
    pub readme: Option<String>,
    pub gitignore: Option<String>,
    pub license: Option<String>,
}

impl NewProjectForm {
    pub fn create(&self, conn: &DbConn) -> Result<Project> {
        NewProject::from_form(conn, &self)
    }
}