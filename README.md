# `format-json-log`

A command line tool to format format json structured log message lines from stdin into ANSI coloured stdout.

## Installation

Install using `cargo install format-json-log`.

## Usage

```
Usage: format-json-log [OPTIONS]

Options:
  -m, --message-pointer <MESSAGE_POINTER>
          JSON pointer to the message string

          [default: /fields/message]

  -t, --timestamp-pointer <TIMESTAMP_POINTER>
          JSON pointer to the timestamp string

          [default: /timestamp]

  -l, --level-pointer <LEVEL_POINTER>
          JSON pointer to the log level string

          [default: /level]

  -f, --format <FORMAT>
          What output format to use

          [default: line]

          Possible values:
          - line: Similar to what `tracing_subscriber` outputs
          - json: Coloured json, with a level heading per line

  -h, --help
          Print help (see a summary with '-h')
```

## `k9s` Plugin

This tool was originally designed to be used to format json log messages as a plugin in `k9s`.

In `~/.config/k9s/plugin.yml` put the following:

```yaml
plugin:
  formatjsonlog:
    shortCut: Shift-L
    description: "attach (pretty)"
    scopes:
      - po
    command: bash
    args:
      - "-i"
      - "-c"
      - "kubectl logs -f $NAME -n $NAMESPACE --context $CONTEXT | format-json-log"
```
