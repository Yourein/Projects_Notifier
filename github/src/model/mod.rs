use std::collections::HashMap;

use chrono::DateTime;
use chrono::FixedOffset;

use crate::query::{ProjectV2Task, get_project_tasks};

type Fields = Vec<PropertyType>;
type PropertyType = get_project_tasks::GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodes;
type SingleSelectFieldType = get_project_tasks::GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldSingleSelectValueField;
type TextValueFieldType = get_project_tasks::GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldTextValueField;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub profile_name: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProjectTask {
    pub created_at: DateTime<FixedOffset>,
    pub created_by: String,
    pub task_id: String,
    pub task_title: String,
    pub assignees: Vec<User>,
    pub status: Option<String>,
    pub size: Option<String>,
    pub priority: Option<String>,
    pub section: Option<String>,
}

impl TryFrom<ProjectV2Task> for ProjectTask {
    type Error = anyhow::Error;

    fn try_from(val: ProjectV2Task) -> Result<Self, Self::Error> {
        let created_at_original = DateTime::parse_from_rfc3339(&val.created_at.0).unwrap();
        let created_at = created_at_original
            .with_timezone(&FixedOffset::east_opt(9*3600).unwrap());
        let created_by = if val.creator.is_some() {
            val.creator.clone().unwrap().login
        } else {
            "".to_string()
        };
        let id = &val.id;
        let fields = val
            .field_values
            .nodes
            .map(|nodes| nodes.into_iter().flatten().collect())
            .unwrap_or_else(Vec::new);

        let assignees = parse_assignees(fields.clone());
        let single_selects: HashMap<String, Option<String>> = parse_single_select(fields.clone());
        let title: String = parse_title(fields.clone());

        Ok(ProjectTask {
            created_at: created_at,
            created_by: created_by,
            task_id: id.clone(),
            task_title: title,
            assignees: assignees,
            status: single_selects.get("Status").unwrap_or_else(|| &None).clone(),
            size: single_selects.get("Size").unwrap_or_else(|| &None).clone(),
            priority: single_selects.get("Priority").unwrap_or_else(|| &None).clone(),
            section: single_selects.get("Section").unwrap_or_else(|| &None).clone(),
        })
    }
}

fn parse_title(fields: Fields) -> String {
    let text_fields = fields
        .into_iter()
        .filter_map(|it| {
            match it {
                PropertyType::ProjectV2ItemFieldTextValue(inner) => {
                    if let TextValueFieldType::ProjectV2Field(field) = inner.field {
                        if field.name == "Title" {
                            return Some(inner.text)
                        } else {
                            return None
                        }
                    } else {
                        return None;
                    };
                }
                _ => { None } 
            }
        })
        .nth(0);

    match text_fields {
        Some(it) => {
            if it.is_some() {
                it.unwrap()
            } else {
                "".to_string()
            }
        }
        None => {
            "".to_string()
        }
    }
}

fn parse_assignees(fields: Fields) -> Vec<User> {
    let assignees_field = fields
        .into_iter()
        .find(|it| 
            match it {
                PropertyType::ProjectV2ItemFieldUserValue(_) => { true }
                _ => { false }
            }
        );
    
    if assignees_field.is_some() {
        if let PropertyType::ProjectV2ItemFieldUserValue(it) = assignees_field.unwrap() {
            it.users.unwrap()
                .nodes.unwrap()
                .into_iter()
                .map(|user| {
                    let raw = user.unwrap();
                    User {
                        user_id: raw.login,
                        profile_name: raw.name,
                    }
                })
                .collect::<Vec<User>>()
        } else {
            vec!()
        }
    } else {
        vec!()
    }
}

fn parse_single_select(fields: Fields) -> HashMap<String, Option<String>> {
    let mut single_selects: HashMap<String, Option<String>> = HashMap::new();
    fields
        .into_iter()
        .filter_map(|it| match it {
            PropertyType::ProjectV2ItemFieldSingleSelectValue(it) => { Some(it) }
            _ => { None }
        })
        .for_each(|it| {
            let field_name = 
                if let SingleSelectFieldType::ProjectV2SingleSelectField(field_settings) = it.field {
                    field_settings.name
                } else {
                    "".to_string()
                };
            let field_value = it.name;

            single_selects.insert(field_name, field_value);
        });
    single_selects
}

/*
struct GetProjectTasksOrganizationProjectV2ItemsNodes {
    pageInfo,
    fieldValues: GetProjectTasksOrganizationProjectV2ItemsNodesFieldValues {
        nodes: Option<Vec<Option<GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodes>>>
    }
}

pub enum GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodes {
    ProjectV2ItemFieldDateValue,
    ProjectV2ItemFieldIterationValue(
        GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldIterationValue,
    ),
    ProjectV2ItemFieldLabelValue,
    ProjectV2ItemFieldMilestoneValue,
    ProjectV2ItemFieldNumberValue,
    ProjectV2ItemFieldPullRequestValue,
    ProjectV2ItemFieldRepositoryValue,
    ProjectV2ItemFieldReviewerValue,
    ProjectV2ItemFieldSingleSelectValue(
        GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldSingleSelectValue,
    ),
    ProjectV2ItemFieldTextValue(
        GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldTextValue,
    ),
    ProjectV2ItemFieldUserValue(
        GetProjectTasksOrganizationProjectV2ItemsNodesFieldValuesNodesOnProjectV2ItemFieldUserValue,
    ),
}
*/