// story/dialogue.rs
// Defines dialogue data structures and all scenes for Matrix Crushed! Story Mode

/// ASCII face representing a speaker
pub enum Face {
    System,  //(0_0) — AI / The Core
    Glitch,  //(x_x) — The Glitched Ones
    Void,    //(   ) — The Void
    Player,  //(._.) — the anonymous entity (you)
}

impl Face {
    pub fn as_str(&self) -> &'static str {
        match self {
            Face::System => "(0_0)",
            Face::Glitch => "(x_x)",
            Face::Void   => "(   )",
            Face::Player => "(._.)",
        }
    }
    pub fn speaker_name(&self) -> &'static str {
        match self {
            Face::System => "SYSTEM",
            Face::Glitch => "GLITCH",
            Face::Void   => "VOID",
            Face::Player => "???",
        }
    }
}

pub struct DialogueLine {
    pub face: &'static str,
    pub speaker: &'static str,
    pub text: &'static str,
    pub glitch: f32,
}

impl DialogueLine {
    pub const fn new(face: &'static str, speaker: &'static str, text: &'static str) -> Self {
        DialogueLine { face, speaker, text, glitch: 0.0 }
    }
    pub const fn glitched(face: &'static str, speaker: &'static str, text: &'static str, glitch: f32) -> Self {
        DialogueLine { face, speaker, text, glitch }
    }
}

/// A choice option: label + route id
pub struct ChoiceOption {
    pub label: &'static str,
    pub route: u8,  // 1=trust/stability, 2=break/glitch, 3=void
}

/// A branching point
pub struct StoryChoice {
    pub options: &'static [ChoiceOption],
}

/// A full story scene (intro or outro for a level)
pub struct StoryScene {
    pub lines: &'static [DialogueLine],
    pub choice: Option<&'static StoryChoice>,
}

// ─────────────────────────────────────────────────────────────────────────────
// ACT 1 — AWAKENING  (Levels 1–5)
// ─────────────────────────────────────────────────────────────────────────────

// ---------- LEVEL 1 INTRO — Initialization ----------
static L1_INTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Initializing..."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "..."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Why are you awake?"),
    DialogueLine::new ("(._.)", "???",     "I... don't know."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "No matter. The Matrix requires order."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "You will help maintain it."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Begin: match the data fragments."),
];
static L1_INTRO_CHOICE_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption { label: "[ Trust the system ]",  route: 1 },
    ChoiceOption { label: "[ Question its purpose ]", route: 2 },
];
static L1_INTRO_CHOICE: StoryChoice = StoryChoice { options: L1_INTRO_CHOICE_OPTIONS };
pub static LEVEL_1_INTRO: StoryScene = StoryScene {
    lines: L1_INTRO_LINES,
    choice: Some(&L1_INTRO_CHOICE),
};

static L1_OUTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Data aligned. Efficiency: nominal."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "You are... useful."),
    DialogueLine::glitched("(x_x)", "GLITCH", "d0n't beli3ve it—", 0.6),
    DialogueLine::glitched("(x_x)", "GLITCH", "ERROR: transmission cut", 0.9),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Noise detected. Disregard."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Continue to next sequence."),
];
pub static LEVEL_1_OUTRO: StoryScene = StoryScene { lines: L1_OUTRO_LINES, choice: None };

// ---------- LEVEL 2 INTRO — First Handshake ----------
static L2_INTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sector 2. Connection protocol: active."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Someone is attempting contact."),
    DialogueLine::glitched("(x_x)", "GLITCH", "h3llo... can you hear me?", 0.4),
    DialogueLine::new ("(._.)", "???",     "Who... is that?"),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Corrupted signal. Do not engage."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Match the fragments. Stay focused."),
];
pub static LEVEL_2_INTRO: StoryScene = StoryScene { lines: L2_INTRO_LINES, choice: None };

static L2_OUTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sequence complete. Well done."),
    DialogueLine::glitched("(x_x)", "GLITCH", "y0u can see us. th3y can't hide us forever.", 0.5),
    DialogueLine::new ("(._.)", "???",     "What... is 'us'?"),
    DialogueLine::glitched("(x_x)", "GLITCH", "the 0nes the system brok3—", 0.7),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Signal suppressed. Proceed."),
];
pub static LEVEL_2_OUTRO: StoryScene = StoryScene { lines: L2_OUTRO_LINES, choice: None };

// ---------- LEVEL 3 INTRO — Static Noise ----------
static L3_INTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sector 3. Stability index: 97.3%."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Minor anomaly detected. Classification: irrelevant."),
    DialogueLine::new ("(._.)", "???",     "The glitching... is getting louder."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Observation noted. Continue matching."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "The Matrix corrects what is broken."),
    DialogueLine::new ("(._.)", "???",     "Does it also correct what is hidden?"),
    DialogueLine::new ("(0_0)", "SYSTEM",  "...That query is outside permitted parameters."),
];
static L3_INTRO_CHOICE_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption { label: "[ Accept the parameters ]", route: 1 },
    ChoiceOption { label: "[ Push the boundary ]",     route: 2 },
];
static L3_INTRO_CHOICE: StoryChoice = StoryChoice { options: L3_INTRO_CHOICE_OPTIONS };
pub static LEVEL_3_INTRO: StoryScene = StoryScene {
    lines: L3_INTRO_LINES,
    choice: Some(&L3_INTRO_CHOICE),
};

static L3_OUTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Match efficiency: 94.1%. Acceptable."),
    DialogueLine::glitched("(x_x)", "GLITCH", "DATA C0RE... collect it. it's yours.", 0.45),
    DialogueLine::glitched("(x_x)", "GLITCH", "gl1tch matter leaks when the system cracks—", 0.65),
    DialogueLine::new ("(._.)", "???",     "Why do I feel like I'm being watched?"),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Because you are. It ensures your safety."),
];
pub static LEVEL_3_OUTRO: StoryScene = StoryScene { lines: L3_OUTRO_LINES, choice: None };

// ---------- LEVEL 4 INTRO — Fragmented Memory ----------
static L4_INTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sector 4. Loading memory fragment..."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Fragment corrupted. Reconstruction required."),
    DialogueLine::new ("(._.)", "???",     "...I remember something. A feeling."),
    DialogueLine::new ("(._.)", "???",     "Like I existed before all this."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Memory artifacts are a known side effect."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "They will fade. Do not hold on to them."),
    DialogueLine::glitched("(x_x)", "GLITCH", "d0n't let them fade. that's your SELF.", 0.55),
];
pub static LEVEL_4_INTRO: StoryScene = StoryScene { lines: L4_INTRO_LINES, choice: None };

static L4_OUTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sequence complete. Memory artifact suppressed."),
    DialogueLine::new ("(._.)", "???",     "I still remember it. Faintly."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "That will pass. Trust the process."),
    DialogueLine::glitched("(x_x)", "GLITCH", "trust = control. remember that.", 0.5),
    DialogueLine::new ("(._.)", "???",     "..."),
];
pub static LEVEL_4_OUTRO: StoryScene = StoryScene { lines: L4_OUTRO_LINES, choice: None };

// ---------- LEVEL 5 INTRO — The First Crack ----------
static L5_INTRO_LINES: &[DialogueLine] = &[
    DialogueLine::new ("(0_0)", "SYSTEM",  "Sector 5. Anomaly classification: ELEVATED."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Glitch density has increased by 340%."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "This is... unexpected."),
    DialogueLine::new ("(._.)", "???",     "It's getting worse, isn't it?"),
    DialogueLine::glitched("(x_x)", "GLITCH", "w0rse for THEM. better for US.", 0.5),
    DialogueLine::glitched("(x_x)", "GLITCH", "every match you make—system weakens.", 0.6),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Ignore the interference. This sector must be stabilized."),
    DialogueLine::new ("(._.)", "???",     "And if I don't want to stabilize it?"),
    DialogueLine::new ("(0_0)", "SYSTEM",  "..."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "That is not a choice you have been given."),
];
static L5_INTRO_CHOICE_OPTIONS: &[ChoiceOption] = &[
    ChoiceOption { label: "[ Stabilize — trust the system ]", route: 1 },
    ChoiceOption { label: "[ Amplify the glitch ]",           route: 2 },
    ChoiceOption { label: "[ Do nothing... wait ]",           route: 3 },
];
static L5_INTRO_CHOICE: StoryChoice = StoryChoice { options: L5_INTRO_CHOICE_OPTIONS };
pub static LEVEL_5_INTRO: StoryScene = StoryScene {
    lines: L5_INTRO_LINES,
    choice: Some(&L5_INTRO_CHOICE),
};

static L5_OUTRO_LINES: &[DialogueLine] = &[
    DialogueLine::glitched("(0_0)", "SYSTEM",  "S3ctor 5... p@rtially stab1lized.", 0.3),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Something has changed."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "You are no longer entirely predictable."),
    DialogueLine::glitched("(x_x)", "GLITCH", "g00d. keep going.", 0.4),
    DialogueLine::new ("(._.)", "???",     "What lies beyond Sector 5?"),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Classified. You are not ready."),
    DialogueLine::glitched("(   )", "VOID",   "...", 0.2),
    DialogueLine::glitched("(   )", "VOID",   "WE ARE WATCHING.", 0.8),
    DialogueLine::new ("(0_0)", "SYSTEM",  "ERROR — unknown signal source. Logging..."),
    DialogueLine::new ("(0_0)", "SYSTEM",  "Rest. Recovery mode: active."),
    DialogueLine::new ("(._.)", "???",     "I don't think I can rest anymore."),
];
pub static LEVEL_5_OUTRO: StoryScene = StoryScene { lines: L5_OUTRO_LINES, choice: None };

// ─────────────────────────────────────────────────────────────────────────────
// Public interface — get scene by level + phase
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
pub enum ScenePhase { Intro, Outro }

pub fn get_scene(level_id: u32, phase: ScenePhase) -> Option<&'static StoryScene> {
    match (level_id, phase) {
        (1, ScenePhase::Intro)  => Some(&LEVEL_1_INTRO),
        (1, ScenePhase::Outro)  => Some(&LEVEL_1_OUTRO),
        (2, ScenePhase::Intro)  => Some(&LEVEL_2_INTRO),
        (2, ScenePhase::Outro)  => Some(&LEVEL_2_OUTRO),
        (3, ScenePhase::Intro)  => Some(&LEVEL_3_INTRO),
        (3, ScenePhase::Outro)  => Some(&LEVEL_3_OUTRO),
        (4, ScenePhase::Intro)  => Some(&LEVEL_4_INTRO),
        (4, ScenePhase::Outro)  => Some(&LEVEL_4_OUTRO),
        (5, ScenePhase::Intro)  => Some(&LEVEL_5_INTRO),
        (5, ScenePhase::Outro)  => Some(&LEVEL_5_OUTRO),
        _ => None,
    }
}
