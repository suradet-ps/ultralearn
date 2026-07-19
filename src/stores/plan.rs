use crate::core::time::{date_plus_days, generate_id, now_iso};
use crate::core::types::{
    ActivityValue, ChecklistItem, Experiment, ExperimentStatus, FeedbackEntry, FeedbackType,
    Flashcard, Plan, PrincipleProgress, RetentionSchedule,
};
use leptos::prelude::*;
use std::sync::OnceLock;

use gloo_storage::{LocalStorage, Storage};

const STORAGE_KEY: &str = "ultralearn-plans";

/// Reactive plan store. Mirrors `src/stores/plan.ts` (Pinia) as a `OnceLock`
/// singleton of `RwSignal`s, installed once inside `mount_to_body`.
#[derive(Debug, Clone, Copy)]
pub struct PlanState {
    pub plans: RwSignal<Vec<Plan>>,
}

impl PlanState {
    fn new() -> Self {
        Self {
            plans: RwSignal::new(load_plans()),
        }
    }

    pub fn get_plan(&self, id: &str) -> Option<Plan> {
        self.plans.get().into_iter().find(|p| p.id == id)
    }

    pub fn create_plan(&self, topic: &str, goal: &str) -> Plan {
        let plan = Plan {
            id: generate_id(),
            topic: topic.to_string(),
            goal: goal.to_string(),
            created_at: now_iso(),
            principles: create_default_progress(),
        };
        let mut plans = self.plans.get_untracked();
        plans.push(plan.clone());
        self.plans.set(plans);
        save_plans(&self.plans.get_untracked());
        plan
    }

    pub fn delete_plan(&self, id: &str) {
        let plans = self
            .plans
            .get_untracked()
            .into_iter()
            .filter(|p| p.id != id)
            .collect::<Vec<_>>();
        self.plans.set(plans);
        save_plans(&self.plans.get_untracked());
    }

    pub fn update_principle_notes(&self, plan_id: &str, principle_id: u32, notes: &str) {
        self.mutate_principle(plan_id, principle_id, |pp| {
            pp.notes = notes.to_string();
        });
    }

    pub fn get_principle_notes(&self, plan_id: &str, principle_id: u32) -> String {
        self.find_progress(plan_id, principle_id)
            .map(|pp| pp.notes)
            .unwrap_or_default()
    }

    pub fn toggle_checklist_item(&self, plan_id: &str, principle_id: u32, item_id: &str) {
        self.mutate_checklist(plan_id, principle_id, item_id, |item| {
            item.checked = !item.checked;
        });
    }

    pub fn add_checklist_item(&self, plan_id: &str, principle_id: u32, text: &str) {
        self.mutate_principle(plan_id, principle_id, |pp| {
            pp.checklist.push(ChecklistItem {
                id: generate_id(),
                text: text.to_string(),
                checked: false,
            });
        });
    }

    pub fn remove_checklist_item(&self, plan_id: &str, principle_id: u32, item_id: &str) {
        self.mutate_principle(plan_id, principle_id, |pp| {
            pp.checklist.retain(|c| c.id != item_id);
        });
    }

    pub fn toggle_principle_completed(&self, plan_id: &str, principle_id: u32) {
        self.mutate_principle(plan_id, principle_id, |pp| {
            pp.completed = !pp.completed;
        });
    }

    pub fn update_activity(
        &self,
        plan_id: &str,
        principle_id: u32,
        key: &str,
        value: ActivityValue,
    ) {
        self.mutate_principle(plan_id, principle_id, |pp| {
            pp.activities.insert(key.to_string(), value);
        });
    }

    pub fn get_progress(&self, plan_id: &str) -> u32 {
        let Some(plan) = self.get_plan(plan_id) else {
            return 0;
        };
        if plan.principles.is_empty() {
            return 0;
        }
        let completed = plan.principles.iter().filter(|p| p.completed).count();
        (completed as f64 / plan.principles.len() as f64 * 100.0).round() as u32
    }

    pub fn get_principle_progress(&self, plan_id: &str, principle_id: u32) -> u32 {
        let Some(pp) = self.find_progress(plan_id, principle_id) else {
            return 0;
        };
        if pp.completed {
            return 100;
        }
        if pp.checklist.is_empty() {
            return 0;
        }
        let checked = pp.checklist.iter().filter(|c| c.checked).count();
        (checked as f64 / pp.checklist.len() as f64 * 100.0).round() as u32
    }

    pub fn get_flashcards(&self, plan_id: &str, principle_id: u32) -> Vec<Flashcard> {
        self.get_activity(plan_id, principle_id, "flashcards")
            .and_then(|v| match v {
                ActivityValue::Flashcards(c) => Some(c),
                ActivityValue::Other(j) => serde_json::from_value(j).ok(),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn add_flashcard(&self, plan_id: &str, principle_id: u32, front: &str, back: &str) {
        let mut cards = self.get_flashcards(plan_id, principle_id);
        cards.push(Flashcard {
            id: generate_id(),
            front: front.to_string(),
            back: back.to_string(),
        });
        self.update_activity(
            plan_id,
            principle_id,
            "flashcards",
            ActivityValue::Flashcards(cards),
        );
    }

    pub fn remove_flashcard(&self, plan_id: &str, principle_id: u32, card_id: &str) {
        let cards = self
            .get_flashcards(plan_id, principle_id)
            .into_iter()
            .filter(|c| c.id != card_id)
            .collect::<Vec<_>>();
        self.update_activity(
            plan_id,
            principle_id,
            "flashcards",
            ActivityValue::Flashcards(cards),
        );
    }

    pub fn get_feedback_log(&self, plan_id: &str, principle_id: u32) -> Vec<FeedbackEntry> {
        self.get_activity(plan_id, principle_id, "feedbackLog")
            .and_then(|v| match v {
                ActivityValue::Feedback(c) => Some(c),
                ActivityValue::Other(j) => serde_json::from_value(j).ok(),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn add_feedback(
        &self,
        plan_id: &str,
        principle_id: u32,
        fb_type: FeedbackType,
        text: &str,
    ) {
        let mut log = self.get_feedback_log(plan_id, principle_id);
        log.push(FeedbackEntry {
            id: generate_id(),
            r#type: fb_type,
            text: text.to_string(),
            date: now_iso(),
        });
        self.update_activity(
            plan_id,
            principle_id,
            "feedbackLog",
            ActivityValue::Feedback(log),
        );
    }

    pub fn get_experiments(&self, plan_id: &str, principle_id: u32) -> Vec<Experiment> {
        self.get_activity(plan_id, principle_id, "experiments")
            .and_then(|v| match v {
                ActivityValue::Experiments(c) => Some(c),
                ActivityValue::Other(j) => serde_json::from_value(j).ok(),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn add_experiment(&self, plan_id: &str, principle_id: u32, hypothesis: &str, method: &str) {
        let mut exps = self.get_experiments(plan_id, principle_id);
        exps.push(Experiment {
            id: generate_id(),
            hypothesis: hypothesis.to_string(),
            method: method.to_string(),
            result: String::new(),
            status: ExperimentStatus::Planned,
        });
        self.update_activity(
            plan_id,
            principle_id,
            "experiments",
            ActivityValue::Experiments(exps),
        );
    }

    pub fn update_experiment(
        &self,
        plan_id: &str,
        principle_id: u32,
        exp_id: &str,
        updates: &Experiment,
    ) {
        let mut exps = self.get_experiments(plan_id, principle_id);
        if let Some(exp) = exps.iter_mut().find(|e| e.id == exp_id) {
            *exp = updates.clone();
        }
        self.update_activity(
            plan_id,
            principle_id,
            "experiments",
            ActivityValue::Experiments(exps),
        );
    }

    pub fn get_schedules(&self, plan_id: &str, principle_id: u32) -> Vec<RetentionSchedule> {
        self.get_activity(plan_id, principle_id, "schedules")
            .and_then(|v| match v {
                ActivityValue::Schedules(c) => Some(c),
                ActivityValue::Other(j) => serde_json::from_value(j).ok(),
                _ => None,
            })
            .unwrap_or_default()
    }

    pub fn add_schedule(&self, plan_id: &str, principle_id: u32, topic: &str) {
        let offsets = [1, 3, 7, 14, 30];
        let dates = offsets
            .iter()
            .map(|d| date_plus_days(*d))
            .collect::<Vec<_>>();
        let mut items = self.get_schedules(plan_id, principle_id);
        items.push(RetentionSchedule {
            id: generate_id(),
            topic: topic.to_string(),
            dates,
            completed_dates: Vec::new(),
        });
        self.update_activity(
            plan_id,
            principle_id,
            "schedules",
            ActivityValue::Schedules(items),
        );
    }

    pub fn toggle_schedule_date(
        &self,
        plan_id: &str,
        principle_id: u32,
        schedule_id: &str,
        date: &str,
    ) {
        let mut items = self.get_schedules(plan_id, principle_id);
        if let Some(s) = items.iter_mut().find(|i| i.id == schedule_id) {
            if let Some(idx) = s.completed_dates.iter().position(|d| d == date) {
                s.completed_dates.remove(idx);
            } else {
                s.completed_dates.push(date.to_string());
            }
        }
        self.update_activity(
            plan_id,
            principle_id,
            "schedules",
            ActivityValue::Schedules(items),
        );
    }

    pub fn export_plan(&self, plan_id: &str) -> String {
        match self.get_plan(plan_id) {
            Some(p) => serde_json::to_string_pretty(&p).unwrap_or_default(),
            None => String::new(),
        }
    }

    pub fn import_plan(&self, json: &str) -> Option<Plan> {
        let plan: Plan = serde_json::from_str(json).ok()?;
        if plan.id.is_empty() || plan.topic.is_empty() || plan.principles.is_empty() {
            return None;
        }
        let mut plan = plan;
        plan.id = generate_id();
        let mut plans = self.plans.get_untracked();
        plans.push(plan.clone());
        self.plans.set(plans);
        save_plans(&self.plans.get_untracked());
        Some(plan)
    }

    // ---- internal helpers ----

    fn find_progress(&self, plan_id: &str, principle_id: u32) -> Option<PrincipleProgress> {
        self.get_plan(plan_id).and_then(|p| {
            p.principles
                .into_iter()
                .find(|pp| pp.principle_id == principle_id)
        })
    }

    fn mutate_principle<F>(&self, plan_id: &str, principle_id: u32, f: F)
    where
        F: FnOnce(&mut PrincipleProgress),
    {
        let mut plans = self.plans.get_untracked();
        for plan in &mut plans {
            if plan.id == plan_id {
                if let Some(pp) = plan
                    .principles
                    .iter_mut()
                    .find(|pp| pp.principle_id == principle_id)
                {
                    f(pp);
                }
                break;
            }
        }
        self.plans.set(plans);
        save_plans(&self.plans.get_untracked());
    }

    fn mutate_checklist<F>(&self, plan_id: &str, principle_id: u32, item_id: &str, f: F)
    where
        F: FnOnce(&mut ChecklistItem),
    {
        let mut plans = self.plans.get_untracked();
        for plan in &mut plans {
            if plan.id == plan_id {
                if let Some(pp) = plan
                    .principles
                    .iter_mut()
                    .find(|pp| pp.principle_id == principle_id)
                    && let Some(item) = pp.checklist.iter_mut().find(|c| c.id == item_id)
                {
                    f(item);
                }
                break;
            }
        }
        self.plans.set(plans);
        save_plans(&self.plans.get_untracked());
    }

    fn get_activity(&self, plan_id: &str, principle_id: u32, key: &str) -> Option<ActivityValue> {
        self.find_progress(plan_id, principle_id)
            .and_then(|pp| pp.activities.get(key).cloned())
    }

    /// Read a free-form text activity (used by the Feynman workspace).
    pub fn get_activity_text(&self, plan_id: &str, principle_id: u32, key: &str) -> String {
        match self.get_activity(plan_id, principle_id, key) {
            Some(ActivityValue::Text(s)) => s,
            Some(ActivityValue::Other(j)) => j.as_str().unwrap_or_default().to_string(),
            _ => String::new(),
        }
    }

    /// Write a free-form text activity (used by the Feynman workspace).
    pub fn set_activity_text(&self, plan_id: &str, principle_id: u32, key: &str, value: &str) {
        self.update_activity(
            plan_id,
            principle_id,
            key,
            ActivityValue::Text(value.to_string()),
        );
    }
}

fn create_default_progress() -> Vec<PrincipleProgress> {
    crate::core::types::principles_data::principles()
        .iter()
        .map(|p| PrincipleProgress {
            principle_id: p.id,
            notes: String::new(),
            checklist: p
                .prompts
                .iter()
                .map(|text| ChecklistItem {
                    id: generate_id(),
                    text: (*text).to_string(),
                    checked: false,
                })
                .collect(),
            activities: std::collections::HashMap::new(),
            completed: false,
        })
        .collect()
}

fn load_plans() -> Vec<Plan> {
    LocalStorage::get::<Vec<Plan>>(STORAGE_KEY).unwrap_or_default()
}

fn save_plans(plans: &[Plan]) {
    let _ = LocalStorage::set(STORAGE_KEY, plans);
}

static PLAN: OnceLock<PlanState> = OnceLock::new();

pub fn install() {
    let _ = PLAN.set(PlanState::new());
}

pub fn use_plan() -> PlanState {
    *PLAN.get().expect("PlanState not initialized")
}
