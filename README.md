# frencli

A fast, powerful command-line batch file renaming tool inspired by the "Multi-Rename Tool" in Total Commander.

**📋 [Changelog](CHANGELOG.md)** - See what's new in each version  
**📖 [Usage Example](USAGE_EXAMPLE.md)** - Complete workflow demonstration

## Features

- **Safe by Default**: Always shows a preview of changes before performing them.
- **Powerful Patterns**: Support for filename, extension, counters, and metadata placeholders.
- **Modifiers**: Built-in support for lowercasing, uppercasing, title casing, and regex replacement.
- **Undo Facility**: Easily reverse your last batch of changes if you make a mistake.
- **Collision Protection**: Skips existing target files by default.
- **Multi-Subcommand Support**: Subcommands can be specified in any order for easier editing.

## Installation

### Via Cargo (Recommended)

```bash
cargo install frencli
```

### From Source

```bash
git clone https://github.com/byezy/frencli.git
cd frencli
cargo build --release
cp ./target/release/fren ~/.local/bin/
```

## Usage

```bash
fren <SUBCOMMAND> [OPTIONS]
```

### Subcommands

- `list`: List files matching patterns
- `validate`: Validate a rename pattern
- `make`: Generate a preview from a pattern
- `rename`: Directly rename files
- `template`: Template operations (`list` or `use`)
- `undo`: Undo operations (`check` or `apply`)
- `audit`: View audit log entries

### Subcommand Order Flexibility

**Subcommands can be specified in any order** - the execution order is determined internally based on logical dependencies. This design makes it much easier to edit command lines, especially when working with different file sets.

**Tip**: If you are going to run a rename pattern multiple times on different file sets, place `list` at the end of your command for easier editing.

```bash
# Easy to edit - just change the patterns at the end
fren make "%N_backup.%E" rename --yes list /some/path/*.txt

# Same command, different files - just edit the end
fren make "%N_backup.%E" rename --yes list /some/other/*.jpg
```

All of these are equivalent:
- `fren list *.txt make "%N.%E" rename`
- `fren make "%N.%E" list *.txt rename`
- `fren rename make "%N.%E" list *.txt`

The internal execution order is always: `list` → `make`/`template --use` → `validate` → `rename`

### Examples

**List files:**
```bash
# List files (shows just filenames)
fren list "*.txt"

# List with full paths
fren list "*.txt" --fullpath

# List recursively
fren list "*.txt" --recursive
```

**Rename files:**
```bash
# Preview a rename
fren list "*.jpg" make "Vacation_%C3.%E"

# Apply the rename
fren list "*.jpg" make "Vacation_%C3.%E" rename --yes

# Use a template
fren list "*.jpg" template --use photo-date rename --yes
```

## Renaming Patterns

Patterns use the `%` character as a prefix for tokens. All tokens are case-insensitive (e.g., `%N` is the same as `%n`).

### Placeholders

| Token | Description | Example (file.txt) |
| :--- | :--- | :--- |
| `%N` | Filename without extension | `file` |
| `%E` | Extension without the dot | `txt` |
| `%F` | Full filename (name + extension) | `file.txt` |
| `%C` | Counter (starts at 1) | `1`, `2`, ... |
| `%C3` | Counter with padding (3 digits) | `001`, `002`, ... |
| `%P` | Immediate parent directory name | `Documents` |
| `%P1-3` | Substring of parent directory | `Doc` |
| `%D` | Current date (YYYY-MM-DD) | `2025-12-18` |
| `%H` | Current time (HH-MM-SS) | `14-30-05` |
| `%FD` | File modification date | `2025-12-10` |
| `%FH` | File modification time | `09-15-00` |

### Substring Selection

You can extract parts of the name or extension using `start-end` indices (1-indexed). Use a double hyphen `--` to count from the end.

- `%N1-3`: Chars 1 to 3 of the name.
- `%N5-`: Chars from index 5 to the end of the name.
- `%N-5`: Chars from the beginning up to index 5.
- `%N--3`: The name minus the last 3 characters (shorthand for from beginning to 3rd from end).
- `%N3--4`: Chars starting from index 3 up to the 4th character from the end.
- `%E1-2`: Chars 1 to 2 of the extension.

### Modifiers

Modifiers apply makeations to the filename. **Order matters!** Modifiers are processed **left-to-right** as they appear in the pattern, and they operate on the **accumulated result** at the point where they are encountered.

- `%L`: Lowercase the entire accumulated result.
- `%U`: Uppercase the entire accumulated result.
- `%T`: Title case the entire accumulated result (capitalizes after spaces, dots, dashes, underscores).
- `%M`: Trim leading and trailing whitespace from the accumulated result.
- `%R/old/new`: Replace occurrences of `old` with `new` in the accumulated result. Supports multiple delimiters: `/`, `|`, `:`, `,`, `@`.
- `%X/pattern/new`: **Regex** replacement in the accumulated result. Supports capturing groups and standard regex syntax.

#### How Modifiers Work

Pattern processing happens in two phases:

1. **Placeholder Expansion**: Placeholders like `%N`, `%E`, `%C` are replaced with their values
2. **Modifier Application**: Modifiers like `%L`, `%U`, `%R` are applied to the accumulated result

**Key Rule**: When a modifier is encountered, it applies to **everything accumulated so far** in the result string, not just what comes after it.

**Example: Modifier Before Placeholder**
```bash
# Pattern: %L%N.%E
# File: Photo_001.JPG
# Result: photo_001.jpg
```
- `%L` sets lowercase mode
- `%N` adds "Photo_001" → becomes "photo_001" (lowercased immediately)
- `.` adds literal dot
- `%E` adds "JPG" → becomes "jpg" (still in lowercase mode)

**Example: Modifier After Placeholder**
```bash
# Pattern: %N%L.%E
# File: Photo_001.JPG
# Result: photo_001.jpg
```
- `%N` adds "Photo_001" (unchanged initially)
- `%L` applies lowercase to accumulated result → "Photo_001" becomes "photo_001"
- `.` adds literal dot
- `%E` adds "JPG" → becomes "jpg" (lowercase mode continues)

**Note**: Both examples produce the same result because `%L` affects everything after it. When placed before `%N`, it lowercases as text is added. When placed after `%N`, it lowercases the accumulated result.

**Example: Order Matters - Multiple Modifiers**
```bash
# Pattern: %U%N%L.%E
# File: photo_001.jpg
# Result: photo_001.jpg
```
- `%U` sets uppercase mode
- `%N` adds "photo_001" → becomes "PHOTO_001"
- `%L` applies lowercase to accumulated result → "PHOTO_001" becomes "photo_001"
- `.` adds literal dot
- `%E` adds "jpg" (lowercase mode continues)

**Compare with**: `%L%N%U.%E` would produce `PHOTO_001.JPG` (lowercase first, then uppercase) - **different result!**

### Pattern Examples

**Simple Case Modification**
```bash
fren list "*.JPG" make "%L%N.%U%E" rename --yes
```

**Organizing by Parent Folder**
```bash
fren list "*" make "%P_%C2_%N.%E" rename --yes
```

**Title Casing**
```bash
fren list "*.mp3" make "%T%N.%E" rename --yes
```

**Replacement**
```bash
fren list "*.txt" make "%R/_/-%N.%E" rename --yes
```

## Safety

`frencli` will **never** overwrite a file unless you explicitly provide the `--overwrite` flag. If a target filename already exists, `frencli` will print a warning and skip that specific file during execution.

### Undo

The `undo` subcommand allows you to reverse the very last batch of renames performed in the current directory. It uses a hidden `.fren_history.json` file to keep track of changes.

```bash
# Check what can be undone
fren undo --check

# Apply undo
fren undo --apply

# Apply undo without confirmation
fren undo --apply --yes
```

If any of the files involved in the undo have been moved, deleted, or replaced by another process since the rename, `frencli` will notify you and skip reversing those specific files to prevent data loss.

## License

This project is licensed under the MIT License.
