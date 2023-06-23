# `format-json-log`

A command line tool to format format json structured log message lines from stdin into ANSI coloured stdout.

```
Usage: format-json-log [OPTIONS]

Options:
  -m, --message-pointer <MESSAGE_POINTER>
          JSON pointer to the message string [default: /fields/message]
  -t, --timestamp-pointer <TIMESTAMP_POINTER>
          JSON pointer to the timestamp string [default: /timestamp]
  -l, --level-pointer <LEVEL_POINTER>
          JSON pointer to the log level string [default: /level]
  -f, --format <FORMAT>
          [default: line] [possible values: line, json]
  -h, --help
          Print help
```
