use github::model::ProjectTask;

#[allow(dead_code)]
pub(crate) struct ProjectTaskShrink {
    pub task_id: String,
    pub task_title: String,
}

impl TryFrom<ProjectTask> for ProjectTaskShrink {
    type Error = anyhow::Error;

    fn try_from(val: ProjectTask) -> Result<Self, Self::Error> {
        Ok(ProjectTaskShrink {
            task_id: val.task_id,
            task_title: val.task_title
        })
    }
}