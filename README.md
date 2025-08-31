# convmit

Generate conventional commit messages using Claude AI.

## Installation

```bash
just install
```

## Setup

First, set your Anthropic API key:

```bash
convmit --set-api-key "your-api-key-here"
```

Get an API key at: https://console.anthropic.com/settings/keys

## Usage

### Generate and commit automatically (default)

First, stage the changes you want to commit. Then run `convmit` to generate a conventional commit message based on the changes you want to commit.

```bash
convmit
```

### Generate message only (no commit)
```bash
convmit --no-commit
```

### Specify a different model
```bash
convmit --model sonnet-4
```

Available models: `opus-4-1`, `opus-4`, `sonnet-4`, `sonnet-3-7`, `haiku-3-5`, `haiku-3`

## How it works

1. Analyzes your staged git files and changes
2. Sends the context to Claude AI
3. Generates a conventional commit message
4. Optionally commits with the generated message

## Configuration

Config is stored at `~/.config/convmit/config.toml`

