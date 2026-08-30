//! Rule-based planner: decides what to surface to the user and when.
//! Deterministic, aging-driven; no ML.

use chrono::NaiveDate;

use crate::models::{Commitment, Status};

#[derive(Debug, Clone)]
pub enum PlanAction {
    /// show it in the main feed / fire a notification
    SurfaceNow,
    /// keep it out of the way until `until`
    Snooze { until: Option<NaiveDate> },
    /// overdue and aging — stronger reminder
    EscalateReminder,
    /// resolved long enough ago; hide from active views
    Archive,
}

#[derive(Debug, Clone)]
pub struct PlannerConfig {
    /// open undated commitments younger than this stay quiet
    pub quiet_days: i64,
    /// open undated commitments older than this surface
    pub surface_after_days: i64,
    /// days past expected_date before escalation kicks in
    pub escalate_after_overdue_days: i64,
    /// days after resolution before archiving
    pub archive_resolved_after_days: i64,
    /// dated commitments surface this many days before due date
    pub pre_due_surface_days: i64,
}

impl Default for PlannerConfig {
    fn default() -> Self {
        Self {
            quiet_days: 2,
            surface_after_days: 3,
            escalate_after_overdue_days: 7,
            archive_resolved_after_days: 30,
            pre_due_surface_days: 1,
        }
    }
}

fn parse_iso(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()
}

fn created_date(c: &Commitment) -> NaiveDate {
    // created_at is RFC3339; take the date part
    c.created_at
        .split('T')
        .next()
        .and_then(parse_iso)
        .unwrap_or_else(|| today_fallback())
}

fn updated_date(c: &Commitment) -> NaiveDate {
    c.last_updated_at
        .split('T')
        .next()
        .and_then(parse_iso)
        .unwrap_or_else(|| today_fallback())
}

fn today_fallback() -> NaiveDate {
    chrono::Utc::now().date_naive()
}

/// Decide what should happen to `c` on `today`.
pub fn plan(c: &Commitment, today: NaiveDate, cfg: &PlannerConfig) -> PlanAction {
    match c.status {
        Status::Resolved => {
            let age = (today - updated_date(c)).num_days();
            if age >= cfg.archive_resolved_after_days {
                PlanAction::Archive
            } else {
                PlanAction::Snooze { until: None }
            }
        }
        Status::Snoozed => PlanAction::Snooze { until: None },
        Status::Overdue => {
            let overdue_days = c
                .expected_date
                .as_deref()
                .and_then(parse_iso)
                .map(|d| (today - d).num_days())
                .unwrap_or(0);
            if overdue_days >= cfg.escalate_after_overdue_days {
                PlanAction::EscalateReminder
            } else {
                PlanAction::SurfaceNow
            }
        }
        Status::Open => match c.expected_date.as_deref().and_then(parse_iso) {
            Some(due) => {
                let days_until = (due - today).num_days();
                if days_until < 0 {
                    PlanAction::SurfaceNow
                } else if days_until <= cfg.pre_due_surface_days {
                    PlanAction::SurfaceNow
                } else {
                    PlanAction::Snooze { until: Some(due) }
                }
            }
            None => {
                let age = (today - created_date(c)).num_days();
                if age >= cfg.surface_after_days {
                    PlanAction::SurfaceNow
                } else if age <= cfg.quiet_days {
                    PlanAction::Snooze { until: None }
                } else {
                    PlanAction::SurfaceNow
                }
            }
        },
    }
}
