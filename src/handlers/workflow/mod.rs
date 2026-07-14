// src/handlers/workflow/mod.rs

use crate::api::workflows::{diagnostic, pnp, replacement};
use crate::commands::workflow::{
    DiagnosticCommands, PnpCommands, ReplacementCommands, WorkflowCommands,
};
use crate::helpers::{command_utils, output};
use chrono::DateTime;
use log::error;
use prettytable::{row, Table};

pub fn handle_workflow_command(subcommand: WorkflowCommands) {
    match subcommand {
        WorkflowCommands::Pnp { subcommand } => handle_pnp(subcommand),
        WorkflowCommands::Diagnostic { subcommand } => handle_diagnostic(subcommand),
        WorkflowCommands::Replacement { subcommand } => handle_replacement(subcommand),
    }
}

// ── PnP ─────────────────────────────────────────────────────────────────────

fn handle_pnp(subcommand: PnpCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            PnpCommands::List { limit, offset } => {
                match pnp::list_pnp_workflows(&ctx.config, &ctx.token, limit, offset).await {
                    Ok(workflows) if workflows.is_empty() => {
                        println!("No PnP workflows found.");
                    }
                    Ok(workflows) => {
                        if output::is_json() {
                            output::print_json(&workflows);
                        } else {
                            println!("\nPnP Workflows ({} returned):", workflows.len());
                            let mut t = Table::new();
                            t.add_row(row![FbFy => "ID", "Name", "Type", "State", "Use State", "Added"]);
                            for w in &workflows {
                                t.add_row(row![
                                    w.id.as_deref().unwrap_or("—"),
                                    w.name.as_deref().unwrap_or("—"),
                                    w.workflow_type.as_deref().unwrap_or("—"),
                                    w.state.as_deref().unwrap_or("—"),
                                    w.use_state.as_deref().unwrap_or("—"),
                                    fmt_ts(w.added_on),
                                ]);
                            }
                            t.printstd();
                        }
                    }
                    Err(e) => error!("Failed to list PnP workflows: {}", e),
                }
            }
            PnpCommands::Get { id } => {
                match pnp::get_pnp_workflow_by_id(&ctx.config, &ctx.token, &id).await {
                    Ok(w) => {
                        if output::is_json() {
                            output::print_json(&w);
                        } else {
                            println!("\nPnP Workflow: {}", w.name.as_deref().unwrap_or(&id));
                            println!("  ID:          {}", w.id.as_deref().unwrap_or("—"));
                            println!("  Type:        {}", w.workflow_type.as_deref().unwrap_or("—"));
                            println!("  State:       {}", w.state.as_deref().unwrap_or("—"));
                            println!("  Use State:   {}", w.use_state.as_deref().unwrap_or("—"));
                            println!("  Description: {}", w.description.as_deref().unwrap_or("—"));
                            println!("  Added:       {}", fmt_ts(w.added_on));
                            println!("  Start:       {}", fmt_ts(w.start_time));
                            println!("  End:         {}", fmt_ts(w.end_time));
                            if let Some(tasks) = &w.tasks {
                                if !tasks.is_empty() {
                                    println!("\n  Tasks:");
                                    let mut t = Table::new();
                                    t.add_row(row![Fy => "Seq", "Name", "Type", "State", "Duration"]);
                                    for task in tasks {
                                        let duration = match (task.start_time, task.end_time) {
                                            (Some(s), Some(e)) => format!("{}ms", e - s),
                                            _ => "—".to_string(),
                                        };
                                        t.add_row(row![
                                            task.task_seq_no.map(|n| n.to_string()).as_deref().unwrap_or("—"),
                                            task.name.as_deref().unwrap_or("—"),
                                            task.task_type.as_deref().unwrap_or("—"),
                                            task.state.as_deref().unwrap_or("—"),
                                            duration,
                                        ]);
                                    }
                                    t.printstd();
                                }
                            }
                        }
                    }
                    Err(e) => error!("Failed to get PnP workflow '{}': {}", id, e),
                }
            }
            PnpCommands::Count => {
                match pnp::get_pnp_workflow_count(&ctx.config, &ctx.token).await {
                    Ok(count) => println!("Total PnP workflows: {}", count),
                    Err(e) => error!("Failed to get PnP workflow count: {}", e),
                }
            }
        }
        Ok(())
    });
}

// ── Diagnostic ───────────────────────────────────────────────────────────────

fn handle_diagnostic(subcommand: DiagnosticCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            DiagnosticCommands::List { status, limit, offset } => {
                match diagnostic::list_diagnostic_workflows(
                    &ctx.config,
                    &ctx.token,
                    status.as_deref(),
                    limit,
                    offset,
                )
                .await
                {
                    Ok(workflows) if workflows.is_empty() => {
                        println!("No diagnostic workflows found.");
                    }
                    Ok(workflows) => {
                        if output::is_json() {
                            output::print_json(&workflows);
                        } else {
                            println!("\nDiagnostic Validation Workflows ({} returned):", workflows.len());
                            let mut t = Table::new();
                            t.add_row(row![FbFy => "ID", "Name", "Status", "Validation Status", "Submitted", "End"]);
                            for w in &workflows {
                                t.add_row(row![
                                    w.id.as_deref().unwrap_or("—"),
                                    w.name.as_deref().unwrap_or("—"),
                                    w.run_status.as_deref().unwrap_or("—"),
                                    w.validation_status.as_deref().unwrap_or("—"),
                                    fmt_ts(w.submit_time),
                                    fmt_ts(w.end_time),
                                ]);
                            }
                            t.printstd();
                        }
                    }
                    Err(e) => error!("Failed to list diagnostic workflows: {}", e),
                }
            }
            DiagnosticCommands::Get { id } => {
                match diagnostic::get_diagnostic_workflow(&ctx.config, &ctx.token, &id).await {
                    Ok(w) => {
                        if output::is_json() {
                            output::print_json(&w);
                        } else {
                            println!("\nDiagnostic Workflow: {}", w.name.as_deref().unwrap_or(&id));
                            println!("  ID:                {}", w.id.as_deref().unwrap_or("—"));
                            println!("  Run Status:        {}", w.run_status.as_deref().unwrap_or("—"));
                            println!("  Validation Status: {}", w.validation_status.as_deref().unwrap_or("—"));
                            println!("  Description:       {}", w.description.as_deref().unwrap_or("—"));
                            println!("  Submitted:         {}", fmt_ts(w.submit_time));
                            println!("  Started:           {}", fmt_ts(w.start_time));
                            println!("  Ended:             {}", fmt_ts(w.end_time));
                            if let Some(ids) = &w.validation_set_ids {
                                println!("  Validation Sets:   {}", ids.join(", "));
                            }
                        }
                    }
                    Err(e) => error!("Failed to get diagnostic workflow '{}': {}", id, e),
                }
            }
            DiagnosticCommands::Count => {
                match diagnostic::get_diagnostic_workflow_count(&ctx.config, &ctx.token).await {
                    Ok(count) => println!("Total diagnostic workflows: {}", count),
                    Err(e) => error!("Failed to get diagnostic workflow count: {}", e),
                }
            }
            DiagnosticCommands::Run { name, description, validation_sets } => {
                let set_ids: Vec<String> = validation_sets
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                match diagnostic::submit_diagnostic_workflow(
                    &ctx.config,
                    &ctx.token,
                    &name,
                    description.as_deref(),
                    set_ids,
                )
                .await
                {
                    Ok(result) => {
                        if output::is_json() {
                            output::print_json(&result);
                        } else {
                            println!("Diagnostic workflow submitted.");
                            println!("  ID:  {}", result.id.as_deref().unwrap_or("—"));
                            println!("  URL: {}", result.url.as_deref().unwrap_or("—"));
                            println!("\nUse `workflow diagnostic get <id>` to poll status.");
                        }
                    }
                    Err(e) => error!("Failed to submit diagnostic workflow: {}", e),
                }
            }
        }
        Ok(())
    });
}

// ── Replacement ──────────────────────────────────────────────────────────────

fn handle_replacement(subcommand: ReplacementCommands) {
    command_utils::execute_with_context(|ctx| async move {
        match subcommand {
            ReplacementCommands::List => {
                match replacement::list_replacement_workflows(&ctx.config, &ctx.token).await {
                    Ok(items) if items.is_empty() => {
                        println!("No device replacement workflows found.");
                    }
                    Ok(items) => {
                        if output::is_json() {
                            output::print_json(&items);
                        } else {
                            println!("\nDevice Replacement Workflows ({} found):", items.len());
                            let mut t = Table::new();
                            t.add_row(row![FbFy =>
                                "Faulty Device", "Faulty Serial", "Replacement Serial",
                                "Status", "Created", "Completed"
                            ]);
                            for r in &items {
                                t.add_row(row![
                                    r.faulty_device_name.as_deref().unwrap_or("—"),
                                    r.faulty_device_serial_number.as_deref().unwrap_or("—"),
                                    r.replacement_device_serial_number.as_deref().unwrap_or("—"),
                                    r.replacement_status.as_deref().unwrap_or("—"),
                                    fmt_ts(r.creation_time),
                                    fmt_ts(r.replacement_time),
                                ]);
                            }
                            t.printstd();
                        }
                    }
                    Err(e) => error!("Failed to list replacement workflows: {}", e),
                }
            }
            ReplacementCommands::Deploy { faulty_serial, replacement_serial } => {
                println!(
                    "Deploying device replacement: {} → {}",
                    faulty_serial, replacement_serial
                );
                match replacement::deploy_replacement_workflow(
                    &ctx.config,
                    &ctx.token,
                    &faulty_serial,
                    &replacement_serial,
                )
                .await
                {
                    Ok(task_id) => {
                        println!("Replacement workflow submitted.");
                        println!("  Task ID: {}", task_id);
                        println!("\nUse `show task get {}` to track progress.", task_id);
                    }
                    Err(e) => error!("Failed to deploy replacement workflow: {}", e),
                }
            }
        }
        Ok(())
    });
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn fmt_ts(ts: Option<i64>) -> String {
    ts.and_then(|ms| DateTime::from_timestamp_millis(ms))
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "—".to_string())
}
