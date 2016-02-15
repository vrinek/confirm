# `confirm`

A very simple command line program to confirm a chained action. It simply prompts the user and it exits cleanly for a positive response.

## Usage

### bash

```
confirm && dangerous_command
# OR
confirm "Dangerous command ahead! Continue?" && dangerous_command
```

### fish

```
confirm; and dangerous_command
# OR
confirm "Dangerous command ahead! Continue?"; and dangerous_command
```
