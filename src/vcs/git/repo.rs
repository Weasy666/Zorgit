use crate::DbConn;
use crate::models::{SourceEntry, SourceLine, User};
use super::{cmd, diff, Server};
use crate::vcs::{self, Diff, SourceEntries, VersionControl};
use crate::utils::IntoOption;
use git2::{self, TreeWalkMode, TreeWalkResult, ObjectType, Oid, BranchType, RepositoryInitOptions, Signature};
use std::collections::{BTreeSet, BTreeMap};
use std::path::{Path, PathBuf};
use std::str::{self, FromStr};
use std::io::Write;
use anyhow::Result;

pub struct Repository {
    name: String,
    repo: git2::Repository,
    path: PathBuf,
}

impl VersionControl for Repository {
    type Output = Self;
    type Server = Server;

    fn server(&self) -> Server {
        Server{}
    }

    fn create<P: AsRef<Path>>(path: P, projectname: &str) -> Result<Self> {
        let path = path.as_ref().join(projectname);
        if !path.is_dir() {
            std::fs::create_dir_all(&path)?;
        }

        let mut opts = RepositoryInitOptions::new();
        opts.bare(true)
            .no_reinit(true);

        let repo = git2::Repository::init_opts(&path, &opts)
            .map(|repo| Repository {
                    name: projectname.to_string(),
                    path,
                    repo,
            })?;
        Ok(repo)
    }

    fn init<P: AsRef<Path>>(&self, tmp_dir: P, user: &User) -> Result<()> {        
        let sig = Signature::now(&user.username, &user.email.to_string()).unwrap();

        //TODO: init commit with different readmes and license and .gitignore
        let filename = "README.md";
        let init_repo = git2::Repository::clone(self.path.to_str().unwrap(), tmp_dir.as_ref())?;

        let tmp_dir = tmp_dir.as_ref().join(filename);
        let mut file = std::fs::File::create(tmp_dir)?;
        file.write_all(b"# ")?;
        file.write_all(&self.name.clone().into_bytes())?;

        let mut index = init_repo.index()?;
        index.add_path(&PathBuf::from_str(filename).unwrap())?;
        let tree_id = index.write_tree()?;
        let tree = init_repo.find_tree(tree_id)?;

        init_repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
        let mut remote = init_repo.find_remote("origin")?;
        remote.connect(git2::Direction::Push)?;
        if remote.connected() {
            let mut callbacks = git2::RemoteCallbacks::new();
            callbacks.push_update_reference(|name, status| {
                println!("name: {} - remote status: {}", name, status.unwrap_or("Everything ok"));
                Ok(())
            });
            let mut push_ops = git2::PushOptions::new();
            push_ops.remote_callbacks(callbacks);
            
            remote.push(&["refs/heads/master"], Some(&mut push_ops))?;
            remote.disconnect()?;
        }

        Ok(())
    }

    fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        Ok(Repository {
            name: path.as_ref().file_name().unwrap().to_str().unwrap().to_string(),
            path: path.as_ref().into(),
            repo: git2::Repository::open(path)?,
        })
    }

    fn last_commit_in_branch(&self, conn: &DbConn, branch_name: &str) -> Result<vcs::Commit> {
        let branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;

        let commit = branch.get()
            .resolve()?
            .peel_to_commit()?;
        Ok(vcs::Commit::from((conn, commit)))
    }
    
    fn branch_entry_by_path<P: AsRef<Path>>(&self, conn: &DbConn, branch_name: &str, path: P) -> Result<Option<SourceEntries>> {
        let branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        let tree = branch.get().peel_to_tree()?;
        let pth = path.as_ref().to_str().unwrap().replace("\\", "/");
        let entry = match tree.get_path(Path::new(&pth)) {
            Ok(e) => e,
            Err(err) if err.code() == git2::ErrorCode::NotFound => return Ok(None),
            Err(err) => Err(err)?,
        };
        let mut entries = Vec::new();
        match entry.kind() {
            Some(git2::ObjectType::Tree) => {
                let folder_tree = self.repo.find_tree(entry.id())?;
                entries = self.entries_for_tree(conn, &folder_tree, branch_name, path.as_ref().to_str().unwrap())?;
            }
            Some(git2::ObjectType::Blob) => {
                let blob = self.repo.find_blob(entry.id())?;
                let name = path.as_ref().file_name().unwrap().to_str().unwrap();
                let mut file = SourceEntry {
                        id: blob.id().to_string(),
                        extension: path.as_ref().extension().map(|s| s.to_str()).flatten().map(ToString::to_string),
                        name: name.to_string(),
                        size: blob.size(),
                        root: path.as_ref().parent().unwrap_or(Path::new("")).to_path_buf(),
                        is_file: true,
                        last_commit: None,
                        content: None,
                    };
                if blob.is_binary() {
                    file.add_line(SourceLine::new_binary(None, None, char::default(), None))
                }
                else {
                    if let Ok(content) = str::from_utf8(blob.content()) {
                        let mut line_num = 0;
                        for line in content.lines() {
                            line_num += 1;
                            file.add_line(SourceLine::new_plain(None, Some(line_num), char::default(), Some(line)));
                        }
                    }
                }
                entries.push((PathBuf::from(name), file));
            }
            _ => {}
        }
        
        Ok(entries.into_option())
    }

    fn raw_branch_entry_by_path<P: AsRef<Path>>(&self, branch_name: &str, path: P) -> Result<Option<Vec<u8>>> {
        let branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        let tree = branch.get().peel_to_tree()?;
        let pth = path.as_ref().to_str().unwrap().replace("\\", "/");
        let tree_entry = tree.get_path(Path::new(&pth))?;
        let data = self.repo.find_blob(tree_entry.id())
            .map(|blob| Some(blob.content().to_owned()))?;
        Ok(data)
    }

    fn default_branch(&self) -> Result<Option<String>> {
        let head = self.repo.head()?;
        let mut default_branch = None;
        if head.is_branch() {
            default_branch = head.shorthand().map(ToString::to_string);
        }
        Ok(default_branch)
    }

    fn branches(&self) -> Result<Option<Vec<String>>> {
        let branches = self.repo.branches(Some(BranchType::Local))?
            .flat_map(|branch| branch )
            .flat_map(|branch| branch.0.name().map(|name| name.map(ToString::to_string)))
            .collect::<Option<Vec<String>>>();
        Ok(branches)
    }

    fn commit_by_id(&self, conn: &DbConn, commit_id: &str) -> Result<vcs::Commit> {
        let sha = Oid::from_str(commit_id)?;
        let commit = self.repo.find_commit(sha)?;
        Ok(vcs::Commit::from((conn, commit)))
    }

    fn branch_last_commit(&self, conn: &DbConn, branch_name: &str) -> Result<vcs::Commit> {
        let commit = self.repo.find_branch(branch_name, git2::BranchType::Local)?
            .get()
            .resolve()?
            .peel_to_commit()?;
        Ok(vcs::Commit::from((conn, commit)))

        // It looks like the peel is enough to yield the last commit.
        // let mut revwalk = self.repo.revwalk().unwrap();
        // revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME);
        // revwalk.push(commit.id()).unwrap();
        // let mut last_commit = Err("Commit not found".to_string());

        // for oid in revwalk {
        //     if let Ok(oid) = oid {
        //         if let Ok(commit) = self.repo.find_commit(oid) {
        //             if with_merge_commits || commit.parent_count() < 2 {
        //                 last_commit = Ok(commit::from_git2(conn, &commit));
        //                 break;
        //             }
        //         }
        //     }
        // }
        
        // last_commit
    }

    fn branch_history(&self, conn: &DbConn, branch_name: &str, with_merge_commits: bool) -> Result<Vec<vcs::Commit>> {
        Repository::_branch_history(&self.repo, branch_name, with_merge_commits)
            .map(|v| v.into_iter().map(|c| vcs::Commit::from((conn, c))).collect::<Vec<_>>())
    }

    fn branch_commits_count(&self, branch_name: &str) -> Result<usize> {
        let commit = self.repo.find_branch(branch_name, git2::BranchType::Local)?
            .get()
            .resolve()?
            .peel_to_commit()?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;
        revwalk.push(commit.id())?;

        let count = revwalk
            .filter(|oid| oid.is_ok())
            .count();
        
        Ok(count)
        
        //TODO: Benchmark if out shelling to commandline is faster
        //cmd::commits_count(&self.path, branch_name)
    }

    fn commit_ancestor_count(&self, commit_id: &str) -> Result<usize> {
        let sha = Oid::from_str(commit_id)?;
        let commit = self.repo.find_commit(sha)?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;
        revwalk.push(commit.id())?;

        let count = revwalk
            .filter(|oid| oid.is_ok())
            .count();
        
        Ok(count)

        //TODO: Benchmark if shelling out to commandline is faster
        //cmd::commits_count(&self.path, commit_id)
    }

    fn commit_associated_branches(&self, commit_id: &str) -> Result<Vec<String>> {
        let sha = Oid::from_str(commit_id)?;
        let branches = self.branches()?;
        if let Some(branches) = branches {
            let mut brnches = Vec::with_capacity(branches.len());

            for branch_name in branches {
                let branch = self.repo.find_branch(&branch_name, git2::BranchType::Local)?;
                let commit = branch.get()
                    .resolve()?
                    .peel_to_commit()?;

                let mut revwalk = self.repo.revwalk()?;
                revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;
                revwalk.push(commit.id())?;

                for oid in revwalk {
                    if let Ok(oid) = oid {
                        if oid == sha {
                            brnches.push(branch_name.clone());
                        }
                    }
                }
            }

            Ok(brnches)
        }
        else {
            Err(anyhow!("Could not find any associated branches for Commit: {}", commit_id))
        }

        //TODO: Benchmark if shelling out to commandline is faster
        //cmd::commit_associated_branches(&self.path, commit_id)
    }

    /// Returns all entries of the given tree. Each tree represents the not recursive content of a folder.
    /// Meaning a folder present in the given tree is not evaluated and its entries need to be retrieved independently.
    fn branch_entries(&self, conn: &DbConn, branch_name: &str) -> Result<Option<SourceEntries>> {
        let branch = self.repo.find_branch(branch_name, git2::BranchType::Local)?;
        let tree = branch.get().peel_to_tree()?;

        self.entries_for_tree(conn, &tree, branch_name, "")
            .map(|v| v.into_option())
    }

    fn diff_to_parent(&self, conn: &DbConn, commit: &vcs::Commit) -> Result<Diff> {
        let mut parent = None;
        if let Some(oid) = &commit.parents_id() {
            let parent_oid = Oid::from_str(oid.get(0).unwrap()).unwrap();
            parent = self.repo.find_commit(parent_oid).map(|c| vcs::Commit::from((conn, c))).ok();
        }

        diff::diff_from_to(&self.repo, Some(commit), parent.as_ref())
    }

    fn diff_from_to(&self, from: &vcs::Commit, to: &vcs::Commit) -> Result<Diff> {  
        diff::diff_from_to(&self.repo, Some(from), Some(to))
    }

    fn calc_size(&self) -> Result<usize> {
        // let index = self.repo.index();
        // if index.is_err() {
        //     return 0;
        // }
        // let index = index.unwrap();
        // let index_len = index.len();
        // println!("Index len for size calc is: {}", index_len);
        // index.iter().map(|entry| {
        //     entry.file_size as usize
        // })
        // .sum()
        cmd::repo_size(&self.path)
    }
}

impl Repository {
    // can maybe be made faster by passing a DiffOptions.pathspec to diff_tree_to_tree. See 'libgit2' Slack channel 'git2-rs' message from 'Alexander von Gluck IV'
    fn entries_for_tree<P: AsRef<Path>>(&self, conn: &DbConn, tree: &git2::Tree<'_>, branch_name: &str, root: P) -> Result<SourceEntries> {
        // Folders and files are separe collections, because this way we get an explorer like sorted list for nearly free
        let mut folders = BTreeMap::new();
        let mut files = BTreeMap::new();
        
        tree.walk(TreeWalkMode::PreOrder, |parent, entry| {
            let name = entry.name().expect("Could not get filename of tree entry.").to_string();
            // We just want to see the entries of the parent level, so we ignore everything with a parent aka that is in a directory
            if parent.is_empty() {
                match entry.kind() {
                    Some(ObjectType::Tree) => { 
                        folders.insert(PathBuf::from(&name), SourceEntry {
                                id: entry.id().to_string(),
                                extension: None,
                                name,
                                size: 0,
                                root: root.as_ref().to_path_buf(),
                                is_file: false,
                                last_commit: None,
                                content: None,
                            });
                        },
                    Some(ObjectType::Blob) => {
                        let blob = self.repo.find_blob(entry.id()).unwrap();
                        let mut file = SourceEntry {
                                id: entry.id().to_string(),
                                extension: name.split('.').last().map(ToString::to_string),
                                name: name.clone(),
                                size: blob.size(),
                                root: root.as_ref().to_path_buf(),
                                is_file: true,
                                last_commit: None,
                                content: None,
                            };
                        if blob.is_binary() {
                            file.add_line(SourceLine::new_binary(None, None, char::default(), None))
                        }
                        files.insert(PathBuf::from(&name), file);
                        },
                    _ => (),
                }
            }

            TreeWalkResult::Ok
        })?;

        let commits = Repository::_branch_history(&self.repo, branch_name, false)?;
        let mut visited = BTreeSet::new();
        let mut file_commits = BTreeMap::new();
        let root = root.as_ref().to_path_buf();
        for commit in commits {
            let mut parent = None;
            if commit.parent_ids().len() > 0 {
                let parent_oid = commit.parent_ids().next().expect("There should be at least one parent.");
                parent = self.repo.find_commit(parent_oid)?.into();
            }

            let diff = diff::_diff_from_to(&self.repo, Some(&commit), parent.as_ref())?;
            for file in diff.files {
                // if file got deleted, name is empty and we need to use old_name, because
                // it's technically still a change that occured in this commit
                let filename = file.name.as_ref().unwrap_or_else(|| file.old_name.as_ref().unwrap()).to_string();
                let filename = PathBuf::from(filename);
                if filename.starts_with(&root) {
                    let root_rel_filename = filename.strip_prefix(&root).expect("Prefix should be there, we checked for it");
                    let root_rel_filename = root_rel_filename.to_path_buf();
                    // don't need the entries in folders, just the folders and entries in the trees root level
                    let key = root_rel_filename.components().next().unwrap().as_os_str();
                    let key = PathBuf::from(key);
                    if !visited.contains(&key) && visited.len() < (folders.len() + files.len()) {
                        if !file_commits.contains_key(&key) && (folders.contains_key(&key) || files.contains_key(&key)) {
                            // We start from newest entry in commit tree, so at this point we know that
                            // the commit we found is the newest and we can stop looking for this file
                            visited.insert(key.clone());
                            file_commits.insert(key, commit.clone());
                        }
                    }
                }
            }
            if visited.len() >= (folders.len() + files.len()) {
                break;
            }
        }

        let mut entries = Vec::new();
        for folder in folders {
            let mut entry = folder.1;
            entry.last_commit = vcs::Commit::from((conn, file_commits.get(&folder.0).expect("No commit found for folder").to_owned())).into();
            entries.push((folder.0, entry));
        }
        for file in files {
            let mut entry = file.1;
            entry.last_commit = vcs::Commit::from((conn, file_commits.get(&file.0).expect(&format!("No commit found for file: {:?}", &file.0)).to_owned())).into();
            entries.push((file.0, entry));
        }

        Ok(entries)
    }

    fn _branch_history<'a>(repo: &'a git2::Repository, branch_name: &str, with_merge_commits: bool) -> Result<Vec<git2::Commit<'a>>> {
        let commit = repo.find_branch(branch_name, git2::BranchType::Local)?
            .get()
            .resolve()?
            .peel_to_commit()?;

        let mut revwalk = repo.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::TIME)?;
        revwalk.push(commit.id())?;

        let commits = revwalk
            .flatten()
            .flat_map(|oid| repo.find_commit(oid))
            .filter(|commit| with_merge_commits || commit.parent_count() < 2)
            .collect::<Vec<_>>();
        
        Ok(commits)
    }
}