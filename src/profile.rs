use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{controller::Controller, team_number::TeamNumber};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub team: TeamNumber,
    pub logs: PathBuf,
    pub controller: Controller,
}

impl Profile {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn team(&self) -> TeamNumber {
        self.team.clone()
    }

    pub fn logs(&self) -> PathBuf {
        self.logs.clone()
    }

    pub fn controller(&self) -> Controller {
        self.controller.clone()
    }
}
