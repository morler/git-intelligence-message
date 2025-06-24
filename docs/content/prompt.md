# Prompt Management

View and edit the AI prompt used for generating commit message, for description and subject respectively:

```bash
# View current prompt
gim prompt

# Open the prompt files in default file manager for editing
gim prompt --edit

# Edit a specific prompt file with default editor
gim prompt --edit --prompt diff

# Edit a specific prompt file with custom editor, 
# like 'code', 'vim' or any other text editor available on your Mac
gim prompt --edit --prompt subject --editor code
```

The `-prompt` option can take these params:

- `d`, `diff`, `diff_prompt` for summarizing file changes, which will be used as the commit description.
- `s`, `subject`, `subject_prompt` for generating the commit subject based on the summary of file changes.

