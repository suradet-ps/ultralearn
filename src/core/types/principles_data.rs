#[derive(Debug, Clone)]
pub struct PrincipleData {
  pub id: u32,
  pub name: String,
  pub tagline: String,
  pub color: String,
  pub description: String,
  pub prompts: Vec<&'static str>,
}

/// The 9 Ultralearning principles. Mirrors `src/types/principles-data.ts`.
pub fn principles() -> &'static [PrincipleData] {
  use std::sync::OnceLock;
  static PRINCIPLES: OnceLock<Vec<PrincipleData>> = OnceLock::new();
  PRINCIPLES.get_or_init(|| {
        vec![
            PrincipleData {
                id: 1,
                name: "Metalearning".into(),
                tagline: "Determine What, Why and How".into(),
                color: "#6366f1".into(),
                description: "Before diving in, map out the subject. Break it into Concepts, Facts, and Procedures. Benchmark how others learn it, then emphasise what aligns with your goal and exclude what doesn't.".into(),
                prompts: vec![
                    "What exactly do you want to learn? Be specific — not \"learn Rust\" but \"build a CLI tool in Rust that parses CSV files\".",
                    "Why do you want to learn this? How will you use it?",
                    "What are the core Concepts you need to understand?",
                    "What Facts need to be memorised?",
                    "What Procedures need to be practised?",
                    "How have others learned this? (courses, curricula, books)",
                    "What can you emphasise or exclude based on your goal?",
                ],
            },
            PrincipleData {
                id: 2,
                name: "Focus".into(),
                tagline: "Eliminate distractions, build concentration".into(),
                color: "#8b5cf6".into(),
                description: "Identify what distracts you and eliminate it. Use techniques like Pomodoro to maintain high-intensity focus. If the task feels too hard, make it easier — but not so easy that you get bored.".into(),
                prompts: vec![
                    "What are your main distractions? (phone, environment, noise)",
                    "What time of day are you most focused?",
                    "How long can you currently sustain deep focus?",
                    "What environment do you need to create?",
                    "What is your focus strategy? (Pomodoro, time blocks, etc.)",
                ],
            },
            PrincipleData {
                id: 3,
                name: "Directness".into(),
                tagline: "Learn by doing the thing itself".into(),
                color: "#06b6d4".into(),
                description: "Transfer learning is hard. Overcome it by practising directly. If learning a language, speak it. If learning math, do problems. If learning programming, build projects.".into(),
                prompts: vec![
                    "What is the most direct way to practise this skill?",
                    "What project can you build that forces you to use what you're learning?",
                    "How can you immerse yourself in the target environment?",
                    "What simulation or flight-simulator approach can you use?",
                    "Are you willing to go overkill — full immersion for a period?",
                ],
            },
            PrincipleData {
                id: 4,
                name: "Drill".into(),
                tagline: "Find and attack your rate-limiting factor".into(),
                color: "#f97316".into(),
                description: "Identify what's holding you back — the rate-limiting factor. Then drill it in isolation. Use time slicing, cognitive components, copycat, magnifying glass, or prerequisite chaining methods.".into(),
                prompts: vec![
                    "What is the weakest part of your performance?",
                    "What sub-skill can you isolate and drill?",
                    "Which drill method will you use? (time slicing, copycat, magnifying glass, prerequisite chaining)",
                    "How will you practise this drill regularly?",
                ],
            },
            PrincipleData {
                id: 5,
                name: "Retrieval".into(),
                tagline: "Test yourself before you're ready".into(),
                color: "#ec4899".into(),
                description: "Difficulty is desirable. Test yourself actively — flashcards, free recall, self-generated challenges. Retrieving information strengthens memory far more than re-reading.".into(),
                prompts: vec![
                    "What flashcards do you need to create?",
                    "After each study session, what can you recall without looking at notes?",
                    "What self-generated challenges can you set?",
                    "Can you do closed-book learning — create a concept map from memory?",
                ],
            },
            PrincipleData {
                id: 6,
                name: "Feedback".into(),
                tagline: "Get signal, not noise".into(),
                color: "#14b8a6".into(),
                description: "Seek immediate, actionable feedback. Distinguish outcome feedback (did it work?), informational feedback (what went wrong?), and corrective feedback (how to fix it). Cancel the noise.".into(),
                prompts: vec![
                    "What feedback do you need right now?",
                    "Where can you get corrective feedback (not just outcome)?",
                    "How can you reduce noise and focus on the signal?",
                    "Are you at the right difficulty sweet spot?",
                    "How can you get feedback faster?",
                ],
            },
            PrincipleData {
                id: 7,
                name: "Retention".into(),
                tagline: "Remember what you learn".into(),
                color: "#f59e0b".into(),
                description: "Use spaced repetition, proceduralisation, overlearning, and mnemonics to fight forgetting. Build habits so learned behaviours become automatic.".into(),
                prompts: vec![
                    "What key facts or concepts need spaced repetition?",
                    "What schedule will you follow? (1 day, 3 days, 7 days, 14 days, 30 days)",
                    "What can you turn into automatic procedures?",
                    "What should you overlearn — practise beyond mastery?",
                    "What mnemonics can you create for hard-to-remember items?",
                ],
            },
            PrincipleData {
                id: 8,
                name: "Intuition".into(),
                tagline: "Build deep understanding, not surface recall".into(),
                color: "#a855f7".into(),
                description: "Use the Feynman Technique: explain the concept as if teaching a beginner. When you get stuck, that's where your understanding gaps are. Go back, fill them, then explain again.".into(),
                prompts: vec![
                    "Choose a concept you want to deeply understand.",
                    "Explain it as if teaching someone who has never heard of it.",
                    "Where did you get stuck? What gaps did you find?",
                    "Go back to your resources and fill those gaps.",
                    "Can you create examples, analogies, or visualisations?",
                ],
            },
            PrincipleData {
                id: 9,
                name: "Experimentation".into(),
                tagline: "Push beyond proficiency to mastery".into(),
                color: "#22c55e".into(),
                description: "Experiment with learning resources, technique, and style. Copy what works, then create your own approach. Combine skills into hybrids. Explore the extremes.".into(),
                prompts: vec![
                    "What learning resources can you try? (video, text, audio, hands-on)",
                    "What techniques can you compare side-by-side?",
                    "What constraints can you introduce to spark creativity?",
                    "What hybrid skills can you combine with this one?",
                    "What extreme experiment can you try?",
                ],
            },
        ]
    })
}

/// Name lookup by id, mirrors the `getPrincipleName` helper in the landing view.
pub fn principle_name(id: u32) -> &'static str {
  principles()
    .iter()
    .find(|p| p.id == id)
    .map_or("", |p| p.name.as_str())
}
