# frencli Usage Example

## Complete Workflow: List → Transform → Rename → Undo

### Step 1: List Files

```bash
$ fren list "*.jpg" "*.txt"
Found 5 matching file(s):
  photo_003.jpg
  photo_002.jpg
  photo_001.jpg
  document_old_2.txt
  document_old.txt
```

### Step 2: Preview Transformation

```bash
$ fren list "*.jpg" "*.txt" transform "Vacation_%C3_%N.%E"
Old Name                                 -> New Name                                
------------------------------------------------------------------------------------
photo_003.jpg                            -> Vacation_001_photo_003.jpg              
photo_002.jpg                            -> Vacation_002_photo_002.jpg              
photo_001.jpg                            -> Vacation_003_photo_001.jpg              
document_old_2.txt                       -> Vacation_004_document_old_2.txt         
document_old.txt                         -> Vacation_005_document_old.txt           

Preview mode. Use 'rename' subcommand to perform the renaming.
```

### Step 3: Apply Rename

```bash
$ fren list "*.jpg" "*.txt" transform "Vacation_%C3_%N.%E" rename --yes
Old Name                                 -> New Name                                
------------------------------------------------------------------------------------
photo_003.jpg                            -> Vacation_001_photo_003.jpg              
photo_002.jpg                            -> Vacation_002_photo_002.jpg              
photo_001.jpg                            -> Vacation_003_photo_001.jpg              
document_old_2.txt                       -> Vacation_004_document_old_2.txt         
document_old.txt                         -> Vacation_005_document_old.txt           

Successfully renamed 5 file(s).
```

### Step 4: Verify Results

```bash
$ ls -1
Vacation_001_photo_003.jpg
Vacation_002_photo_002.jpg
Vacation_003_photo_001.jpg
Vacation_004_document_old_2.txt
Vacation_005_document_old.txt
```

### Step 5: Check Undo Availability

```bash
$ fren undo --check
Checking undo state for 5 renames from 2025-12-20 15:06:14...

5 file(s) can be safely undone.
```

### Step 6: Apply Undo

```bash
$ fren undo --apply --yes
Checking undo state for 5 renames from 2025-12-20 15:06:14...
Successfully reversed 5 renames.
```

### Step 7: Verify Undo

```bash
$ ls -1
document_old_2.txt
document_old.txt
photo_001.jpg
photo_002.jpg
photo_003.jpg
```

