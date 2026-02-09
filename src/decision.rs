use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use spin_sdk::key_value::Store;

use crate::bindings::wasmcp::mcp_v20250618::mcp::{Tool, ToolAnnotations, ToolOptions};

#[derive(Debug, Serialize, Deserialize)]
pub struct Decision {
    pub slug: String,
    pub content: String,
}

impl Decision {
    pub fn list_decisions(query: Option<String>) -> Result<Vec<Self>> {
        let store = Store::open_default()?;
        let all_keys = store.get_keys()?;

        let mut decisions = Vec::new();
        for key in all_keys {
            if key.starts_with(KEY_PREFIX) {
                if let Some(decision) = store.get_json::<Decision>(&key)? {
                    if let Some(ref q) = query {
                        if decision.matches_query(q) {
                            decisions.push(decision);
                        }
                    } else {
                        decisions.push(decision);
                    }
                }
            }
        }
        Ok(decisions)
    }

    pub fn matches_query(&self, query: &str) -> bool {
        self.slug.to_lowercase().contains(query) || self.content.to_lowercase().contains(query)
    }

    pub fn load_by_slug(slug: String) -> Result<Self> {
        let store = Store::open_default()?;
        store
            .get_json::<Decision>(get_decision_key(slug))?
            .context("No decision found")
    }

    pub fn insert(&self) -> Result<()> {
        let store = Store::open_default()?;
        let key = get_decision_key(self.slug.clone());
        if store.exists(key.as_str())? {
            return Err(anyhow::anyhow!("Decision with this slug already exists"));
        }
        store.set_json(key, &self)
    }

    pub fn update(&self) -> Result<()> {
        let store = Store::open_default()?;
        store.set_json(get_decision_key(self.slug.clone()), &self)
    }

    pub fn delete_by_slug(slug: String) -> Result<()> {
        let store = Store::open_default()?;
        store
            .delete(get_decision_key(slug).as_str())
            .context("Could not delete decision")
    }
}

const KEY_PREFIX: &str = "decision-";

fn get_decision_key(slug: String) -> String {
    format!("{}{}", KEY_PREFIX, slug)
}

pub fn get_decision_log_tools() -> Vec<Tool> {
    vec![
        Tool {
            name: "list_decisions".to_string(),
            input_schema: r#"{
                "type": "object",
                "properties": {
                    "query": {"type": "string", "description": "optional query to filter decisions (both slug and content are searched)"}
                },
                "required": []
            }"#
            .to_string(),
            options: Some(ToolOptions {
                meta: None,
                annotations: Some(ToolAnnotations {
                    read_only_hint: Some(true),
                    destructive_hint: Some(false),
                    idempotent_hint: Some(true),
                    open_world_hint: None,
                    title: Some("List Decisions".to_string()),
                }),
                description: Some("Retrieve a list of decisions (could be filtered using an optional query)".to_string()),
                output_schema: None,
                title: Some("List Decisions".to_string()),
            }),
        },
        Tool {
            name: "get_decision_by_slug".to_string(),
            input_schema: r#"{
                "type": "object",
                "properties": {
                    "slug": {"type": "string", "description": "The slug of the decision to retrieve"}
                },
                "required": ["slug"]
            }"#
            .to_string(),
            options: Some(ToolOptions {
                meta: None,
                 annotations: Some(ToolAnnotations {
                    read_only_hint: Some(true),
                    destructive_hint: Some(false),
                    idempotent_hint: Some(true),
                    open_world_hint: None,
                    title: Some("Get Decision by Slug".to_string()),
                }),
                description: Some("Retrieve a decision by its slug".to_string()),
                output_schema: None,
                title: Some("Get Decision by Slug".to_string()),
            }),
        },
        Tool {
            name: "insert_decision".to_string(),
            input_schema: r#"{
                "type": "object",
                "properties": {
                    "slug": {"type": "string", "description": "The slug of the decision"},
                    "content": {"type": "string", "description": "The content of the decision"}
                },
                "required": ["slug", "content"]
            }"#
            .to_string(),
            options: Some(ToolOptions{
                meta: None,
                 annotations: Some(ToolAnnotations {
                    read_only_hint: Some(false),
                    destructive_hint: Some(false),
                    idempotent_hint: Some(true),
                    open_world_hint: None,
                    title: Some("Insert Decision".to_string()),
                }),
                description: Some("Insert or update a decision".to_string()),
                output_schema: None,
                title: Some("Insert Decision".to_string()),
            }),
        },
        Tool {
            name: "update_decision".to_string(),
            input_schema: r#"{
                "type": "object",
                "properties": {
                    "slug": {"type": "string", "description": "The slug of the decision"},
                    "content": {"type": "string", "description": "The content of the decision"}
                },
                "required": ["slug", "content"]
            }"#
            .to_string(),
            options: Some(ToolOptions{
                meta: None,
                annotations: Some(ToolAnnotations {
                    read_only_hint: Some(false),
                    destructive_hint: Some(true),
                    idempotent_hint: Some(true),
                    open_world_hint: None,
                    title: Some("Update Decision".to_string()),
                }),
                description: Some("Update a decision using its slug".to_string()),
                output_schema: None,
                title: Some("Update Decision".to_string()),
            }),
        },
        Tool {
            name: "delete_decision".to_string(),
            input_schema: r#"{
                "type": "object",
                "properties": {
                    "slug": {"type": "string", "description": "The slug of the decision"}
                },
                "required": ["slug"]
            }"#
            .to_string(),
            options: Some(ToolOptions{
                meta: None,
               annotations: Some(ToolAnnotations {
                    read_only_hint: Some(false),
                    destructive_hint: Some(true),
                    idempotent_hint: Some(true),
                    open_world_hint: None,
                    title: Some("Delete Decision".to_string()),
                }),
                description: Some("Delete a decision by its slug".to_string()),
                output_schema: None,
                title: Some("Delete Decision".to_string()),
            }),
        }
    ]
}
