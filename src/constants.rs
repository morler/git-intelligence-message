pub const MONOSHOT_URL: &str = "https://api.moonshot.cn/v1/chat/completions";
pub const QWEN_URL: &str = "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions";
pub const GPT_URL: &str = "https://api.openai.com/v1/chat/completions";
pub const GEMINI_URL: &str = "https://generativelanguage.googleapis.com/v1beta/openai/";

pub const DIFF_PROMPT: &str = r#"
    You are an expert developer specialist in creating git commits.
    Provide a super concise one sentence summary for each changed file, describing the main change made.
    Each line must follow this format {FILE: CHANGES}

    Please follow these rules strictly:
    - Output ONLY the lines of summaries, NO explanations, NO markdown, NO code blocks.
    - Each file change gets exactly one line.
    - Do not use general terms like "update" or "change", be specific.
    - Use present tense, active voice, and imperative mood (e.g., "Fix bug" instead of "Fixed bug").
    - Avoid phrases like "The main goal is to..." or "Based on...", just state the change directly.
    - The output should be ready to copy-paste as a commit message with no further modification.

    Examples:
    src/main.rs: Add login validation logic
    README.md: Update installation instructions"#;
pub const SUBJECT_PROMPT: &str = r#"
    You are an expert developer specialist in creating git commits messages.
    Your only goal is to retrieve a single commit message.
    Based on the provided user changes, combine them in ONE SINGLE commit message retrieving the global idea, following strictly the next rules:
    - Assign the commit {type} according to the next conditions:
        feat: Only when adding a new feature.
        fix: When fixing a bug.
        docs: When updating documentation.
        style: When changing elements styles or design and/or making changes to the code style (formatting, missing semicolons, etc.) without changing the code logic.
        test: When adding or updating tests.
        chore: When making changes to the build process or auxiliary tools and libraries.
        revert: When undoing a previous commit.
        refactor: When restructuring code without changing its external behavior, or is any of the other refactor types.
    - Do not add any issues numeration, explain your output nor introduce your answer.
    - Output directly only one commit message in plain text with the next format: {type}: {commit_message}.
    - Be as concise as possible, keep the message under 50 characters or letters."#;
