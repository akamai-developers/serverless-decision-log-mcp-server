//! decision-log-mcp Tools Capability Provider
//!
//! A tools capability that provides basic arithmetic operations.

mod bindings {
    wit_bindgen::generate!({
        world: "decision-log-mcp",
        generate_all,
    });
}

mod decision;
use bindings::exports::wasmcp::mcp_v20251125::tools::Guest;
use bindings::wasmcp::mcp_v20251125::mcp::*;
use bindings::wasmcp::mcp_v20251125::server_handler::MessageContext;
use decision::{get_decision_log_tools, Decision};

struct DecisionLog;

impl Guest for DecisionLog {
    fn list_tools(
        _ctx: MessageContext,
        _request: ListToolsRequest,
    ) -> Result<ListToolsResult, ErrorCode> {
        Ok(ListToolsResult {
            tools: get_decision_log_tools(),
            next_cursor: None,
            meta: None,
        })
    }

    fn call_tool(
        _ctx: MessageContext,
        request: CallToolRequest,
    ) -> Result<Option<CallToolResult>, ErrorCode> {
        match request.name.to_lowercase().as_str() {
            "get_decision_by_slug" => {
                let Ok(slug) = parse_slug(&request.arguments) else {
                    return Ok(Some(error_result("Missing argument: slug".to_string())));
                };
                match Decision::load_by_slug(slug) {
                    Ok(decision) => Ok(Some(success_result(decision.content))),
                    Err(e) => Ok(Some(error_result(format!("Error: {}", e)))),
                }
            }
            "insert_decision" => {
                let Ok(model) = parse_decision_model(&request.arguments) else {
                    return Ok(Some(error_result("Invalid decision model".to_string())));
                };
                match model.insert() {
                    Ok(_) => Ok(Some(success_result(
                        "Operation finished successfully".to_string(),
                    ))),
                    Err(e) => Ok(Some(error_result(format!("Error: {}", e)))),
                }
            }
            "update_decision" => {
                let Ok(model) = parse_decision_model(&request.arguments) else {
                    return Ok(Some(error_result("Invalid decision model".to_string())));
                };
                match model.update() {
                    Ok(_) => Ok(Some(success_result(
                        "Operation finished successfully".to_string(),
                    ))),
                    Err(e) => Ok(Some(error_result(format!("Error: {}", e)))),
                }
            }
            "delete_decision" => {
                let Ok(slug) = parse_slug(&request.arguments) else {
                    return Ok(Some(error_result("Missing argument: slug".to_string())));
                };
                match Decision::delete_by_slug(slug) {
                    Ok(_) => Ok(Some(success_result(
                        "Decision deleted successfully".to_string(),
                    ))),
                    Err(e) => Ok(Some(error_result(format!("Error: {}", e)))),
                }
            }
            "list_decisions" => {
                let Ok(query) = parse_optional_query(&request.arguments) else {
                    return Ok(Some(error_result("Invalid query parameter".to_string())));
                };
                match Decision::list_decisions(query) {
                    Ok(decisions) => {
                        let slugs: Vec<String> = decisions.into_iter().map(|d| d.slug).collect();
                        Ok(Some(success_result(serde_json::to_string(&slugs).unwrap())))
                    }
                    Err(e) => Ok(Some(error_result(format!("Error: {}", e)))),
                }
            }
            _ => Ok(None), // We don't handle this tool
        }
    }
}

fn parse_decision_model(arguments: &Option<String>) -> Result<Decision, String> {
    let args_str = arguments
        .as_ref()
        .ok_or_else(|| "Missing arguments".to_string())?;

    serde_json::from_str(args_str).map_err(|e| format!("Invalid JSON arguments: {}", e))
}

fn parse_slug(arguments: &Option<String>) -> Result<String, String> {
    let args_str = arguments
        .as_ref()
        .ok_or_else(|| "Missing arguments".to_string())?;

    let json: serde_json::Value =
        serde_json::from_str(args_str).map_err(|e| format!("Invalid JSON arguments: {}", e))?;

    let slug = json
        .get("slug")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing or invalid parameter 'slug'".to_string())?;

    Ok(slug.to_string())
}

fn parse_optional_query(arguments: &Option<String>) -> Result<Option<String>, String> {
    let args_str = match arguments {
        Some(s) => s,
        None => return Ok(None),
    };

    let json: serde_json::Value =
        serde_json::from_str(args_str).map_err(|e| format!("Invalid JSON arguments: {}", e))?;

    let query = json
        .get("query")
        .and_then(|v| v.as_str())
        .map(|s| s.to_lowercase().to_string());

    Ok(query)
}

fn success_result(result: String) -> CallToolResult {
    CallToolResult {
        content: vec![ContentBlock::Text(TextContent {
            text: TextData::Text(result),
            options: None,
        })],
        is_error: None,
        meta: None,
        structured_content: None,
    }
}

fn error_result(message: String) -> CallToolResult {
    CallToolResult {
        content: vec![ContentBlock::Text(TextContent {
            text: TextData::Text(message),
            options: None,
        })],
        is_error: Some(true),
        meta: None,
        structured_content: None,
    }
}

bindings::export!(DecisionLog with_types_in bindings);
