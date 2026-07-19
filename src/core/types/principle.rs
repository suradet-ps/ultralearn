use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
  pub id: u32,
  pub name: String,
  pub tagline: String,
  pub color: String,
  pub description: String,
  pub prompts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
  pub id: String,
  pub topic: String,
  pub goal: String,
  pub created_at: String,
  pub tags: Vec<String>,
  pub focus_sessions: Vec<FocusSession>,
  pub principles: Vec<PrincipleProgress>,
}

/// A single completed focus session, recorded by the Pomodoro timer so the
/// learner can see a "focus this week" streak. Stored per-plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusSession {
  pub id: String,
  /// ISO-8601 timestamp of when the session finished.
  pub finished_at: String,
  /// Focus duration in seconds (the Work interval length).
  pub duration_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipleProgress {
  pub principle_id: u32,
  pub notes: String,
  pub checklist: Vec<ChecklistItem>,
  pub activities: std::collections::HashMap<String, ActivityValue>,
  pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChecklistItem {
  pub id: String,
  pub text: String,
  pub checked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
  pub id: String,
  pub front: String,
  pub back: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEntry {
  pub id: String,
  pub r#type: FeedbackType,
  pub text: String,
  pub date: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FeedbackType {
  Outcome,
  Informational,
  Corrective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
  pub id: String,
  pub hypothesis: String,
  pub method: String,
  pub result: String,
  pub status: ExperimentStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExperimentStatus {
  Planned,
  Running,
  Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionSchedule {
  pub id: String,
  pub topic: String,
  pub dates: Vec<String>,
  pub completed_dates: Vec<String>,
}

/// Because `activities` values are heterogeneous (flashcards, feedback, etc.),
/// we store them as JSON values (`serde_json::Value`) rather than typed maps.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActivityValue {
  Flashcards(Vec<Flashcard>),
  Feedback(Vec<FeedbackEntry>),
  Experiments(Vec<Experiment>),
  Schedules(Vec<RetentionSchedule>),
  Text(String),
  Other(serde_json::Value),
}
