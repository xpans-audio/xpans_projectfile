/*!
A data format combining necessary information to load audio and spatial data of
scenes in the xpans Ecosystem
*/
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/**
Contains paths to audio and spatial data in addition to other identifying
and descriptive information for a spatial audio project.
*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    /// The title of this project
    pub title: String,
    /**
    Path to the audio data, relative to the parent directory of this
    project file
    */
    pub audio: PathBuf,
    /**
    Path to the spatial data, relative to the parent directory of this
    project file
    */
    pub spatial: PathBuf,
    /// A map of names for each audio source (optional)
    #[serde(default)]
    pub source_names: BTreeMap<usize, String>,
}

impl ProjectFile {
    /**
    Converts this `ProjectFile` into a `Project`, which just converts its
    relative paths to absolute paths using the given parent `directory`.
    */
    pub fn into_project(self, parent_directory: &Path) -> Option<Project> {
        // Check if the files exist at the absolute paths:
        parent_directory.join(&self.audio).exists().then_some(())?;
        parent_directory
            .join(&self.spatial)
            .exists()
            .then_some(())?;

        Some(Project {
            title: self.title,
            audio: parent_directory.join(self.audio),
            spatial: parent_directory.join(self.spatial),
            source_names: self.source_names,
        })
    }
}

/// Like `ProjectFile`, but with absolute paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// The title of this project
    pub title: String,
    /// Absolute path to the audio data
    pub audio: PathBuf,
    /// Absolute path to the spatial data
    pub spatial: PathBuf,
    /// A map of names for each audio source (optional)
    #[serde(default)]
    pub source_names: BTreeMap<usize, String>,
}
