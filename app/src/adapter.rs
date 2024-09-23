use github::model::ProjectTask;
use slack::{blocks::{Attachments, SectionBlock, TextBlock}, post::Post, traits::Block};

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

pub(crate) fn create_post_from_task<'a>(task: ProjectTask) -> Post<'a> {
    let mut res = Post::new();
    res.add_text_block("新しいタスクを見ていかなイカ?");
    
    let mut attachments = Attachments::new(Some("#da2751".to_string()));
    attachments.add_block(Block::TextBlock(
        TextBlock::new(task.task_title.clone())
    ));

    let mut sections = SectionBlock::new();
    sections.add_paragraph("作成者", &task.created_by);
    sections.add_paragraph(
        "作成日時",
        &task.created_at.format("%Y/%m/%d %H:%M").to_string()
    );
    
    if task.assignees.len() > 0 {
        let assignees = task.assignees
            .clone()
            .into_iter()
            .map(|it| {
                if it.profile_name.is_some() {
                    it.profile_name.clone().unwrap()
                } else {
                    it.user_id
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        sections.add_paragraph("担当者", &assignees);
    }

    if task.section.is_some() {
        let section = task.section.clone().unwrap();
        sections.add_paragraph("パート", &section);
    }

    if task.status.is_some() {
        let status = task.status.clone().unwrap();
        sections.add_paragraph("状態", &status);
    }

    if task.priority.is_some() {
        let priority = task.priority.clone().unwrap();
        sections.add_paragraph("優先度", &priority);
    }

    if task.size.is_some() {
        let size = task.size.clone().unwrap();
        sections.add_paragraph("見積もり", &size);
    }

    attachments.add_block(
        Block::SectionBlock(
            sections
        )
    );
    res.add_attachment(attachments);

    res
}