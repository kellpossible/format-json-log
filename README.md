# `format-json-log`

A command line tool to format format json structured log message lines from stdin into ANSI coloured stdout.

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
