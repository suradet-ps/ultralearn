import type { Principle } from './principle'

export const PRINCIPLES: Principle[] = [
  {
    id: 1,
    name: 'Metalearning',
    tagline: 'Determine What, Why and How',
    color: '#6366f1',
    description:
      'Before diving in, map out the subject. Break it into Concepts, Facts, and Procedures. Benchmark how others learn it, then emphasise what aligns with your goal and exclude what doesn\'t.',
    prompts: [
      'What exactly do you want to learn? Be specific — not "learn Rust" but "build a CLI tool in Rust that parses CSV files".',
      'Why do you want to learn this? How will you use it?',
      'What are the core Concepts you need to understand?',
      'What Facts need to be memorised?',
      'What Procedures need to be practised?',
      'How have others learned this? (courses, curricula, books)',
      'What can you emphasise or exclude based on your goal?',
    ],
  },
  {
    id: 2,
    name: 'Focus',
    tagline: 'Eliminate distractions, build concentration',
    color: '#8b5cf6',
    description:
      'Identify what distracts you and eliminate it. Use techniques like Pomodoro to maintain high-intensity focus. If the task feels too hard, make it easier — but not so easy that you get bored.',
    prompts: [
      'What are your main distractions? (phone, environment, noise)',
      'What time of day are you most focused?',
      'How long can you currently sustain deep focus?',
      'What environment do you need to create?',
      'What is your focus strategy? (Pomodoro, time blocks, etc.)',
    ],
  },
  {
    id: 3,
    name: 'Directness',
    tagline: 'Learn by doing the thing itself',
    color: '#06b6d4',
    description:
      'Transfer learning is hard. Overcome it by practising directly. If learning a language, speak it. If learning math, do problems. If learning programming, build projects.',
    prompts: [
      'What is the most direct way to practise this skill?',
      'What project can you build that forces you to use what you\'re learning?',
      'How can you immerse yourself in the target environment?',
      'What simulation or flight-simulator approach can you use?',
      'Are you willing to go overkill — full immersion for a period?',
    ],
  },
  {
    id: 4,
    name: 'Drill',
    tagline: 'Find and attack your rate-limiting factor',
    color: '#f97316',
    description:
      'Identify what\'s holding you back — the rate-limiting factor. Then drill it in isolation. Use time slicing, cognitive components, copycat, magnifying glass, or prerequisite chaining methods.',
    prompts: [
      'What is the weakest part of your performance?',
      'What sub-skill can you isolate and drill?',
      'Which drill method will you use? (time slicing, copycat, magnifying glass, prerequisite chaining)',
      'How will you practise this drill regularly?',
    ],
  },
  {
    id: 5,
    name: 'Retrieval',
    tagline: 'Test yourself before you\'re ready',
    color: '#ec4899',
    description:
      'Difficulty is desirable. Test yourself actively — flashcards, free recall, self-generated challenges. Retrieving information strengthens memory far more than re-reading.',
    prompts: [
      'What flashcards do you need to create?',
      'After each study session, what can you recall without looking at notes?',
      'What self-generated challenges can you set?',
      'Can you do closed-book learning — create a concept map from memory?',
    ],
  },
  {
    id: 6,
    name: 'Feedback',
    tagline: 'Get signal, not noise',
    color: '#14b8a6',
    description:
      'Seek immediate, actionable feedback. Distinguish outcome feedback (did it work?), informational feedback (what went wrong?), and corrective feedback (how to fix it). Cancel the noise.',
    prompts: [
      'What feedback do you need right now?',
      'Where can you get corrective feedback (not just outcome)?',
      'How can you reduce noise and focus on the signal?',
      'Are you at the right difficulty sweet spot?',
      'How can you get feedback faster?',
    ],
  },
  {
    id: 7,
    name: 'Retention',
    tagline: 'Remember what you learn',
    color: '#f59e0b',
    description:
      'Use spaced repetition, proceduralisation, overlearning, and mnemonics to fight forgetting. Build habits so learned behaviours become automatic.',
    prompts: [
      'What key facts or concepts need spaced repetition?',
      'What schedule will you follow? (1 day, 3 days, 7 days, 14 days, 30 days)',
      'What can you turn into automatic procedures?',
      'What should you overlearn — practise beyond mastery?',
      'What mnemonics can you create for hard-to-remember items?',
    ],
  },
  {
    id: 8,
    name: 'Intuition',
    tagline: 'Build deep understanding, not surface recall',
    color: '#a855f7',
    description:
      'Use the Feynman Technique: explain the concept as if teaching a beginner. When you get stuck, that\'s where your understanding gaps are. Go back, fill them, then explain again.',
    prompts: [
      'Choose a concept you want to deeply understand.',
      'Explain it as if teaching someone who has never heard of it.',
      'Where did you get stuck? What gaps did you find?',
      'Go back to your resources and fill those gaps.',
      'Can you create examples, analogies, or visualisations?',
    ],
  },
  {
    id: 9,
    name: 'Experimentation',
    tagline: 'Push beyond proficiency to mastery',
    color: '#22c55e',
    description:
      'Experiment with learning resources, technique, and style. Copy what works, then create your own approach. Combine skills into hybrids. Explore the extremes.',
    prompts: [
      'What learning resources can you try? (video, text, audio, hands-on)',
      'What techniques can you compare side-by-side?',
      'What constraints can you introduce to spark creativity?',
      'What hybrid skills can you combine with this one?',
      'What extreme experiment can you try?',
    ],
  },
]
