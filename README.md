# convmit

Generate conventional commit messages using Claude AI.

## Installation

```bash
just install
```

## Setup

First, set your Anthropic API key or OpenAI API key:

You can find them at:

- Claude, https://console.anthropic.com/settings/keys
- OpenAI, https://platform.openai.com/api-keys

```bash
# For Claude models
convmit --set-claude-key "your-api-key-here"

# For OpenAI models
convmit --set-openai-key "your-api-key-here"
```

Get an API key at: https://console.anthropic.com/settings/keys

## Usage

### Generate and commit automatically (default)

First, stage the changes you want to commit. Then run `convmit` to generate a conventional commit message based on the changes you want to commit.

```bash
$ convmit
Generated commit message: refactor(code): Improve model display and code formatting
✓ Committed with generated message
```

### Generate message only (no commit)

```bash
convmit --no-commit
Generated commit message: refactor(code): Improve model display and code formatting
```

### Specify a different model

```bash
convmit --model sonnet-4
Generated commit message: refactor(code): Improve model display and code formatting
```

See `convmit --help` for all the models.

## How it works

1. Analyzes your staged git files and changes
2. Sends the context to Claude AI
3. Generates a conventional commit message
4. Optionally commits with the generated message

## Configuration

Config is stored at:

- **macOS** - `~/Library/Application Support/convmit/config.toml`
- **Linux** - `~/.config/convmit/config.toml`
