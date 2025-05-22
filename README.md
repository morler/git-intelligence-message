# Git Intelligence Message (GIM) 🚀

[![Crates.io](https://img.shields.io/crates/v/git-intelligence-message)](https://crates.io/crates/git-intelligence-message)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

An advanced Git commit message generation utility designed to automatically craft high-quality commit messages with precision and sophistication.

## Features

- 🤖 AI-powered commit message generation
- ⚡ Lightning fast Rust implementation
- 🔧 Easy configuration for various AI providers
- 🌍 Multi-language support
- 🔄 Automatic git staging (optional)
- ✏️ Amend previous commits

## Installation

### Using Homebrew (macOS/Linux)

```bash
brew tap davelet/gim
brew install git-intelligence-message
```
or
```bash
brew install davelet/gim/git-intelligence-message
```

### Using Cargo

```bash
cargo install git-intelligence-message
```

### Build from source

```bash
git clone https://github.com/davelet/git-intelligence-message.git
cd git-intelligence-message
cargo install --path .
```

## Command Line Interface

### Basic Usage

```bash
# Generate commit message automatically
gim

# Specify commit title
gim --title "your commit title"

# Stage unstaged changes automatically
gim --auto-add

# Amend the most recent commit
gim --update
```

### Recommended Usage

```bash
# Basic usage - generate commit message for staged changes
gim

# Auto-stage changes and generate commit message
gim -a

# Amend the most recent commit
gim -ap
```

### Command Options

- `-t, --title <STRING>`: Specify the commit message title
- `-a, --auto-add`: Automatically stage all modifications
- `-p, --update`: Amend the most recent commit

### Prompt Management

View and edit the AI prompt templates used for generating commit messages:

```bash
# View current prompt templates
gim prompt

# Open the prompt files in default file manager for editing
gim prompt --edit

# Edit a specific prompt file with default editor
gim prompt --edit --prompt diff

# Edit a specific prompt file with custom editor
gim prompt --edit --prompt subject --editor code
```

Prompt types:
- `d`, `diff`, `diff_prompt`: Diff analysis prompt template
- `s`, `subject`, `subject_prompt`: Commit subject generation prompt template

### AI Configuration

#### Configuration

Utilise the `gim ai` command to configure AI-related parameters:

```bash
# Configure AI model
gim ai --model "your-model-name"

# Set API key
gim ai --apikey "your-api-key"

# Define API endpoint
gim ai --url "your-api-url"

# Set output language
gim ai --language "your-language"
```

> Important: The `--url` parameter only supports OpenAI-compatible API endpoints ,such as OpenAI official or third-party services compatible with OpenAI protocol.

#### AI Configuration Options

- `-m, --model <STRING>`: Specify the AI model to be utilised
- `-k, --apikey <STRING>`: Configure the API key for AI service
- `-u, --url <STRING>`: Set the API endpoint for AI service
- `-l, --language <STRING>`: Define the language for generated commit messages
- `-v, --verbose`: Show verbose output including AI chat content

#### Built-in Model Support

The following model prefixes are supported with their respective default endpoints:

| Model Prefix   | Service Provider | Default Endpoint |
|----------------|------------------|------------------|
| `gpt-*`       | OpenAI           | `https://api.openai.com/v1/chat/completions` |
| `moonshot-*`  | Moonshot AI      | `https://api.moonshot.cn/v1/chat/completions` |
| `qwen-*`      | Alibaba Qwen     | `https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions` |
| `gemini-*`    | Google Gemini    | `https://generativelanguage.googleapis.com/v1beta/openai/` |
| `doubao-*`    | ByteDance Doubao | `https://ark.cn-beijing.volces.com/api/v3/chat/completions` |
| `glm-*`       | THUDM GLM        | `https://open.bigmodel.cn/api/paas/v4/chat/completions` |
| `deepseek-*`  | DeepSeek         | `https://api.deepseek.com/chat/completions` |
| `qianfan-*`   | Baidu Qianfan    | `https://qianfan.baidubce.com/v2/chat/completions` |


You can use any model name starting with these prefixes, and the corresponding endpoint will be used automatically (so you don't need to set `--url`).

### Update

Check for updates and install the latest version:

```bash
gim update
```

Force update even if you're on the latest version:

```bash
gim update --force
```

The application will automatically check for updates when you run it.
