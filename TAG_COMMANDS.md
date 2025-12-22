# Git Tagging Commands for v0.1.3

## Step 1: Stage and Commit Changes

```bash
cd frencli

# Stage all changes
git add .

# Commit with version message
git commit -m "chore: bump version to 0.1.3

- Add comprehensive test coverage (executor_tests.rs, help_tests.rs)
- Refactor main.rs: extract executor module (407 → 124 lines)
- Update documentation (README, CHANGELOG)
- All modules now have corresponding test files"
```

## Step 2: Create Annotated Tag

```bash
# Create annotated tag (recommended for releases)
git tag -a v0.1.3 -m "Release version 0.1.3

- Added comprehensive test coverage for all modules
- Major code refactoring: extracted executor module
- Improved code organization and maintainability
- All 186 tests passing"
```

## Step 3: Push Commit and Tag

```bash
# Push the commit first
git push origin main

# Push the tag
git push origin v0.1.3

# Or push both at once
git push origin main --tags
```

## Alternative: Lightweight Tag (if you prefer)

```bash
# Lightweight tag (no message)
git tag v0.1.3

# Push tag
git push origin v0.1.3
```

## Verify Tag

```bash
# List tags
git tag -l

# Show tag details
git show v0.1.3
```

## One-Liner (if already committed)

```bash
git tag -a v0.1.3 -m "Release v0.1.3" && git push origin main --tags
```

