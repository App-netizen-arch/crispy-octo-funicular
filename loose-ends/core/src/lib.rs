//! Loose Ends symbolic core.
//!
//! Layer separation (Codesym-derived): the neural extractor (Stage 3) only
//! *proposes* candidates; everything that becomes a fact passes through this
//! symbolic layer — typed storage, rule-based direction/date/party logic,
//! deterministic planner — and finally through explicit user confirmation.

pub mod dates;
#[cfg(feature = "neural")]
pub mod neural;
pub mod models;
pub mod party;
pub mod planner;
pub mod rules;
pub mod store;

use chrono::NaiveDate;

pub use models::*;
pub use planner::{plan, PlanAction, PlannerConfig};
pub use store::{NewCommitment, Store};

/// Result of ingesting one raw input through the rule-based path.
#[derive(Debug, Clone)]
pub struct IngestReport {
    pub entry_source_id: i64,
    pub draft_ids: Vec<i64>,
    pub n_candidates: usize,
}

impl Store {
    /// Rule-only ingest (also the circuit-breaker fallback target).
    pub fn ingest_text_rules(&self, text: &str, today: NaiveDate) -> rusqlite::Result<IngestReport> {
        let src = self.add_entry_source(RawInputType::Text)?;
        let cands = rules::extract_rules(text, today);
        let mut ids = Vec::new();
        for c in &cands {
            let id = self.add_draft(
                &c.description,
                c.direction,
                c.expected_date.map(|d| d.to_string()).as_deref(),
                c.party_guess.as_deref(),
                Provenance::RuleExtracted,
                &c.confidence,
                Some(src),
            )?;
            ids.push(id);
        }
        Ok(IngestReport {
            entry_source_id: src,
            n_candidates: cands.len(),
            draft_ids: ids,
        })
    }

    /// The two primary views (spec Stage 4), with planner actions attached.
    pub fn view(&self, dir: Direction, today: NaiveDate, cfg: &PlannerConfig) -> rusqlite::Result<Vec<(Commitment, PlanAction)>> {
        self.refresh_overdue(&today.to_string())?;
        let items = self.list_open(dir)?;
        Ok(items.into_iter().map(|c| {
            let a = plan(&c, today, cfg);
            (c, a)
        }).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> Store {
        Store::open_in_memory().unwrap()
    }

    #[test]
    fn manual_crud_roundtrip() {
        let s = store();
        let id = s
            .create_commitment(NewCommitment {
                description: "pay Dave 15 quid",
                direction: Direction::UserOwes,
                expected_date: None,
                owed_by_party: Some("user"),
                owed_to_party: Some("Dave"),
                provenance: Provenance::Manual,
                confidence: Confidence::default(),
            })
            .unwrap();

        let c = s.get_commitment(id).unwrap().unwrap();
        assert_eq!(c.description, "pay Dave 15 quid");
        assert_eq!(c.direction, Direction::UserOwes);
        assert_eq!(c.status, Status::Open);
        assert_eq!(c.owed_to.as_deref(), Some("Dave"));
        assert_eq!(c.source_provenance, Provenance::Manual);

        s.update_commitment_fields(id, Some("pay Dave 20"), Some(None), None).unwrap();
        let c2 = s.get_commitment(id).unwrap().unwrap();
        assert_eq!(c2.description, "pay Dave 20");
        assert_eq!(c2.expected_date, None);

        s.snooze_commitment(id).unwrap();
        assert_eq!(s.get_commitment(id).unwrap().unwrap().status, Status::Snoozed);
        s.reopen_commitment(id).unwrap();

        s.resolve_commitment(id, Some("paid up")).unwrap();
        let c3 = s.get_commitment(id).unwrap().unwrap();
        assert_eq!(c3.status, Status::Resolved);
    }

    #[test]
    fn unclear_direction_cannot_become_fact_directly() {
        // schema-level guarantee: CHECK constraint rejects anything but the
        // two firm directions
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        let err = conn.execute(
            "INSERT INTO commitment(description, direction, created_at, last_updated_at,
             source_provenance, confidence_json) VALUES ('x','unclear','t','t','manual','{}')",
            [],
        );
        assert!(err.is_err());
    }

    #[test]
    fn rules_ingest_creates_drafts_not_facts() {
        let s = store();
        let today = NaiveDate::from_ymd_opt(2026, 8, 26).unwrap();
        let rep = s
            .ingest_text_rules(
                "ok so: pay back Lena the 20 euros by thursday, and remind Marco to send me the receipt",
                today,
            )
            .unwrap();
        assert_eq!(rep.n_candidates, 2);
        assert_eq!(s.list_all().unwrap().len(), 0, "no facts before confirmation");
        let drafts = s.list_drafts().unwrap();
        assert_eq!(drafts.len(), 2);
        assert!(drafts.iter().all(|d| d.source_provenance == Provenance::RuleExtracted));

        // confirm both with edits applied by "the user"
        let mut facts = Vec::new();
        for d in &drafts {
            if let Some(fid) = s.confirm_draft(d.id, None, Some(Direction::UserOwes), None, None).unwrap() {
                facts.push(fid);
            }
        }
        assert_eq!(facts.len(), 2);
        assert!(s.list_drafts().unwrap().is_empty());
        assert_eq!(s.list_all().unwrap().len(), 2);
    }

    #[test]
    fn views_and_aging() {
        let s = store();
        let cfg = PlannerConfig::default();
        let today = NaiveDate::from_ymd_opt(2026, 8, 26).unwrap();

        // old undated -> surface
        let a = s.create_commitment(NewCommitment {
            description: "old promise", direction: Direction::UserOwes,
            expected_date: None, owed_by_party: Some("user"), owed_to_party: None,
            provenance: Provenance::Manual, confidence: Confidence::default(),
        }).unwrap();
        s.conn.execute(
            "UPDATE commitment SET created_at='2026-08-01T00:00:00+00:00' WHERE id=?1",
            [a],
        ).unwrap();

        // dated far future -> snooze
        s.create_commitment(NewCommitment {
            description: "future thing", direction: Direction::UserOwes,
            expected_date: Some("2026-12-01"), owed_by_party: Some("user"), owed_to_party: None,
            provenance: Provenance::Manual, confidence: Confidence::default(),
        }).unwrap();

        // overdue long ago -> escalate
        s.create_commitment(NewCommitment {
            description: "ancient debt", direction: Direction::UserOwes,
            expected_date: Some("2026-08-01"), owed_by_party: Some("user"), owed_to_party: None,
            provenance: Provenance::Manual, confidence: Confidence::default(),
        }).unwrap();

        let view = s.view(Direction::UserOwes, today, &cfg).unwrap();
        let acts: Vec<String> = view.iter().map(|(_, a)| match a {
            PlanAction::Snooze { .. } => "snooze".to_string(),
            PlanAction::SurfaceNow => "surface".to_string(),
            PlanAction::EscalateReminder => "escalate".to_string(),
            PlanAction::Archive => "archive".to_string(),
        }).collect();
        assert!(acts.contains(&"escalate".to_string()), "{acts:?}");
        assert!(acts.contains(&"snooze".to_string()), "{acts:?}");
        assert!(acts.contains(&"surface".to_string()), "{acts:?}");

        // direction split works
        let owed_view = s.view(Direction::OwedToUser, today, &cfg).unwrap();
        assert!(owed_view.is_empty());
    }
}
