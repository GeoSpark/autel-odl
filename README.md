# Open Drone Log Autel Drone Parser

## Disclaimer
This has only been tested with logs from an Autel EVO II v3 drone. Updates and submissions of logs from other drones are
welcome.

## Installation
Simply copy the executable to wherever you like.

## Use
Because Open Drone Log uses file extensions to map files to parsers, when you copy your log files from the controller,
you need to rename them to add the file extension `.autel`.

Create or update your `parsers.json` file as per the [documentation](https://github.com/arpanghosh8453/open-dronelog/blob/main/docs/custom_parsers.md#2-configuration-parsersjson):

```JSON
{
  "mappings": {
    "autel": {
      "command": "/absolute/path/to/autel_odl",
      "args": ["$INPUT", "$OUTPUT"]
    }
  }
}
```

## Features
Most of the log format has been reverse engineered, and can be found in the [Kaitai Struct](https://kaitai.io/) file
`autel_parser.ksy`. If changes are made to the `.ksy` file, the code can be regenerated with:

$ `kaitai-struct-compiler -t rust . autel_drone_log.ksy`

When the flight mode changes, an event message is recorded and displayed at the appropriate time in the UI. There is
probably more interesting stuff in the various flag fields that haven't been decoded yet.
