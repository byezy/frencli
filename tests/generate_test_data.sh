#!/bin/bash
# Generate test data files for frencli tests
# This script creates the test files needed for frencli's test suite
# Test data is generated in the crate root at: frencli/test_data/

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Script is in tests/, test_data is at crate root level
# Path: tests/ -> .. -> crate root -> test_data
TEST_DATA_DIR="$SCRIPT_DIR/../test_data"

# Create test_data directory structure
mkdir -p "$TEST_DATA_DIR"
mkdir -p "$TEST_DATA_DIR/Photos"
mkdir -p "$TEST_DATA_DIR/Photos/Vacation"
mkdir -p "$TEST_DATA_DIR/Photos/Vacation/2024"
mkdir -p "$TEST_DATA_DIR/Documents"
mkdir -p "$TEST_DATA_DIR/Documents/Projects"
mkdir -p "$TEST_DATA_DIR/Documents/Archive"
mkdir -p "$TEST_DATA_DIR/Media"
mkdir -p "$TEST_DATA_DIR/Media/Music"
mkdir -p "$TEST_DATA_DIR/Media/Videos"
mkdir -p "$TEST_DATA_DIR/Backups"
mkdir -p "$TEST_DATA_DIR/Backups/2024"
mkdir -p "$TEST_DATA_DIR/Backups/2024/January"
mkdir -p "$TEST_DATA_DIR/Backups/2024/February"
mkdir -p "$TEST_DATA_DIR/Logs"
mkdir -p "$TEST_DATA_DIR/Logs/Application"
mkdir -p "$TEST_DATA_DIR/Logs/System"
mkdir -p "$TEST_DATA_DIR/Temp"
mkdir -p "$TEST_DATA_DIR/Config"
mkdir -p "$TEST_DATA_DIR/Config/Fonts"
mkdir -p "$TEST_DATA_DIR/Config/Themes"
mkdir -p "$TEST_DATA_DIR/Code"
mkdir -p "$TEST_DATA_DIR/Code/src"
mkdir -p "$TEST_DATA_DIR/Code/tests"
mkdir -p "$TEST_DATA_DIR/Code/docs"

echo "Generating test data files for frencli in $TEST_DATA_DIR..."

# Helper function to create a minimal valid JPEG file
create_jpeg() {
    local filename="$1"
    # Create a minimal valid JPEG (just the header)
    printf '\xFF\xD8\xFF\xE0\x00\x10JFIF\x00\x01\x01\x01\x00H\x00H\x00\x00\xFF\xD9' > "$filename"
}

# Helper function to create a minimal valid PNG file
create_png() {
    local filename="$1"
    # Create a minimal valid PNG (just the header)
    printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\tpHYs\x00\x00\x0b\x13\x00\x00\x0b\x13\x01\x00\x9a\x9c\x18\x00\x00\x00\nIDATx\x9cc\x00\x01\x00\x00\x05\x00\x01\r\n-\xdb\x00\x00\x00\x00IEND\xaeB`\x82' > "$filename"
}

# Helper function to create a minimal valid PDF file
create_pdf() {
    local filename="$1"
    # Create a minimal valid PDF
    echo "%PDF-1.4
1 0 obj
<< /Type /Catalog /Pages 2 0 R >>
endobj
2 0 obj
<< /Type /Pages /Kids [3 0 R] /Count 1 >>
endobj
3 0 obj
<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>
endobj
xref
0 4
0000000000 65535 f 
0000000009 00000 n 
0000000058 00000 n 
0000000115 00000 n 
trailer
<< /Size 4 /Root 1 0 R >>
startxref
178
%%EOF" > "$filename"
}

# Helper function to create a minimal valid TTF file
create_ttf() {
    local filename="$1"
    # Create a minimal file that looks like a TTF (not a real font, but sufficient for file extension tests)
    echo "TTF placeholder content" > "$filename"
}

# ============================================================================
# Root level files (various types and naming patterns)
# ============================================================================

# Image files
create_jpeg "$TEST_DATA_DIR/photo_001.jpg"
create_jpeg "$TEST_DATA_DIR/photo_002.jpg"
create_jpeg "$TEST_DATA_DIR/photo.jpg"
create_png "$TEST_DATA_DIR/image-001.png"
create_png "$TEST_DATA_DIR/image-002.png"

# Text files with different naming patterns
echo "Test content for file with dashes" > "$TEST_DATA_DIR/file-with-dashes-1.txt"
echo "Test content for file with underscores" > "$TEST_DATA_DIR/file_with_underscores_1.txt"
echo "Test content" > "$TEST_DATA_DIR/file1.txt"
echo "Test content" > "$TEST_DATA_DIR/file123.txt"
echo "Test content" > "$TEST_DATA_DIR/file456.txt"
echo "Test content" > "$TEST_DATA_DIR/file789.txt"
echo "Test content" > "$TEST_DATA_DIR/fileWithCamelCase1.txt"
echo "Test content" > "$TEST_DATA_DIR/FileWithPascalCase1.txt"
echo "Test content" > "$TEST_DATA_DIR/output.txt"
echo "Test content" > "$TEST_DATA_DIR/output1.txt"
echo "Test content" > "$TEST_DATA_DIR/output2.txt"
echo "Test content" > "$TEST_DATA_DIR/output3.txt"
echo "Test content" > "$TEST_DATA_DIR/output4.txt"
echo "Test content" > "$TEST_DATA_DIR/Test File.txt"
echo "Test content" > "$TEST_DATA_DIR/Test File 2.txt"
echo "Test content" > "$TEST_DATA_DIR/Test File 3.txt"
echo "Test content" > "$TEST_DATA_DIR/Renamed_File.txt"
echo "Test content" > "$TEST_DATA_DIR/test-file-1.txt"

# Markdown files
echo "# Appendix A" > "$TEST_DATA_DIR/appendix-A.md"
echo "# Appendix B" > "$TEST_DATA_DIR/appendix-B.md"
echo "# Chapter 01" > "$TEST_DATA_DIR/Chapter 01.md"
echo "# Chapter 02" > "$TEST_DATA_DIR/Chapter 02.md"
echo "# Chapter 03" > "$TEST_DATA_DIR/Chapter 03.md"

# Log files
echo "Log entry 1" > "$TEST_DATA_DIR/old_name_1.log"
echo "Log entry 2" > "$TEST_DATA_DIR/old_name_2.log"
echo "Log entry 3" > "$TEST_DATA_DIR/old_name_3.log"

# Temporary files
echo "Test content" > "$TEST_DATA_DIR/ov_test1.tmp"
echo "Test content" > "$TEST_DATA_DIR/ov_test2.tmp"

# Font files
create_ttf "$TEST_DATA_DIR/InterDisplay-Regular.ttf"
create_ttf "$TEST_DATA_DIR/InterDisplay-Bold.ttf"
create_ttf "$TEST_DATA_DIR/InterDisplay-Italic.ttf"
create_ttf "$TEST_DATA_DIR/InterDisplay-Light.ttf"
create_ttf "$TEST_DATA_DIR/InterDisplay-Medium.ttf"

# PDF files
create_pdf "$TEST_DATA_DIR/My Document.pdf"
create_pdf "$TEST_DATA_DIR/Another Document.pdf"

# Data files
echo "Backup data 2024-01-01" > "$TEST_DATA_DIR/backup_2024_01_01.dat"
echo "Backup data 2024-01-02" > "$TEST_DATA_DIR/backup_2024_01_02.dat"

# Files with specific names for test scenarios
echo "Test content" > "$TEST_DATA_DIR/photo_001_test_test_new.jpg"
echo "Test content" > "$TEST_DATA_DIR/photo_002_test_test_new.jpg"
echo "Test content" > "$TEST_DATA_DIR/photo_003_test_test_new.jpg"
echo "Test content" > "$TEST_DATA_DIR/photo_test_test_new.jpg"

# ============================================================================
# Photos directory structure
# ============================================================================

create_jpeg "$TEST_DATA_DIR/Photos/IMG_001.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/IMG_002.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/IMG_003.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/IMG_0842.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/IMG_0843.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/IMG_0844.jpg"
create_png "$TEST_DATA_DIR/Photos/screenshot.png"
create_png "$TEST_DATA_DIR/Photos/thumbnail.png"
create_png "$TEST_DATA_DIR/Photos/icon.png"

# Photos/Vacation
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/beach_001.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/beach_002.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/mountain_001.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/sunset.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/landscape.jpg"

# Photos/Vacation/2024
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/2024/summer_001.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/2024/summer_002.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/2024/winter_001.jpg"
create_jpeg "$TEST_DATA_DIR/Photos/Vacation/2024/spring_001.jpg"

# ============================================================================
# Documents directory structure
# ============================================================================

create_pdf "$TEST_DATA_DIR/Documents/report.pdf"
create_pdf "$TEST_DATA_DIR/Documents/manual.pdf"
create_pdf "$TEST_DATA_DIR/Documents/guide.pdf"
echo "Project notes" > "$TEST_DATA_DIR/Documents/notes.txt"
echo "Meeting minutes" > "$TEST_DATA_DIR/Documents/meeting.txt"
echo "TODO list" > "$TEST_DATA_DIR/Documents/todo.txt"

# Documents/Projects
echo "Project A documentation" > "$TEST_DATA_DIR/Documents/Projects/project_a.txt"
echo "Project B documentation" > "$TEST_DATA_DIR/Documents/Projects/project_b.txt"
echo "Project C documentation" > "$TEST_DATA_DIR/Documents/Projects/project_c.txt"
create_pdf "$TEST_DATA_DIR/Documents/Projects/spec.pdf"
create_pdf "$TEST_DATA_DIR/Documents/Projects/design.pdf"
echo "Requirements" > "$TEST_DATA_DIR/Documents/Projects/requirements.txt"

# Documents/Archive
echo "Archived document 1" > "$TEST_DATA_DIR/Documents/Archive/doc1.txt"
echo "Archived document 2" > "$TEST_DATA_DIR/Documents/Archive/doc2.txt"
echo "Archived document 3" > "$TEST_DATA_DIR/Documents/Archive/doc3.txt"
create_pdf "$TEST_DATA_DIR/Documents/Archive/old_report.pdf"
create_pdf "$TEST_DATA_DIR/Documents/Archive/old_manual.pdf"

# ============================================================================
# Media directory structure
# ============================================================================

# Media/Music
echo "Audio metadata" > "$TEST_DATA_DIR/Media/Music/song1.mp3"
echo "Audio metadata" > "$TEST_DATA_DIR/Media/Music/song2.mp3"
echo "Audio metadata" > "$TEST_DATA_DIR/Media/Music/song3.mp3"
create_jpeg "$TEST_DATA_DIR/Media/Music/album_cover.jpg"
create_png "$TEST_DATA_DIR/Media/Music/artwork.png"

# Media/Videos
echo "Video metadata" > "$TEST_DATA_DIR/Media/Videos/video1.mp4"
echo "Video metadata" > "$TEST_DATA_DIR/Media/Videos/video2.mp4"
echo "Video metadata" > "$TEST_DATA_DIR/Media/Videos/video3.mp4"
create_png "$TEST_DATA_DIR/Media/Videos/thumbnail.png"
create_png "$TEST_DATA_DIR/Media/Videos/poster.png"

# ============================================================================
# Backups directory structure
# ============================================================================

echo "Backup data" > "$TEST_DATA_DIR/Backups/backup_001.dat"
echo "Backup data" > "$TEST_DATA_DIR/Backups/backup_002.dat"
echo "Backup data" > "$TEST_DATA_DIR/Backups/backup_003.dat"

# Backups/2024
echo "Backup data 2024" > "$TEST_DATA_DIR/Backups/2024/backup_2024_01.dat"
echo "Backup data 2024" > "$TEST_DATA_DIR/Backups/2024/backup_2024_02.dat"
echo "Backup data 2024" > "$TEST_DATA_DIR/Backups/2024/backup_2024_03.dat"

# Backups/2024/January
echo "January backup" > "$TEST_DATA_DIR/Backups/2024/January/jan_backup_01.dat"
echo "January backup" > "$TEST_DATA_DIR/Backups/2024/January/jan_backup_02.dat"
echo "January backup" > "$TEST_DATA_DIR/Backups/2024/January/jan_backup_03.dat"

# Backups/2024/February
echo "February backup" > "$TEST_DATA_DIR/Backups/2024/February/feb_backup_01.dat"
echo "February backup" > "$TEST_DATA_DIR/Backups/2024/February/feb_backup_02.dat"
echo "February backup" > "$TEST_DATA_DIR/Backups/2024/February/feb_backup_03.dat"

# ============================================================================
# Logs directory structure
# ============================================================================

echo "Application log entry" > "$TEST_DATA_DIR/Logs/app.log"
echo "System log entry" > "$TEST_DATA_DIR/Logs/system.log"
echo "General log" > "$TEST_DATA_DIR/Logs/general.log"

# Logs/Application
echo "App log 1" > "$TEST_DATA_DIR/Logs/Application/app1.log"
echo "App log 2" > "$TEST_DATA_DIR/Logs/Application/app2.log"
echo "App log 3" > "$TEST_DATA_DIR/Logs/Application/app3.log"
echo "Error log" > "$TEST_DATA_DIR/Logs/Application/error.log"
echo "Debug log" > "$TEST_DATA_DIR/Logs/Application/debug.log"

# Logs/System
echo "System log 1" > "$TEST_DATA_DIR/Logs/System/sys1.log"
echo "System log 2" > "$TEST_DATA_DIR/Logs/System/sys2.log"
echo "System log 3" > "$TEST_DATA_DIR/Logs/System/sys3.log"
echo "Kernel log" > "$TEST_DATA_DIR/Logs/System/kernel.log"
echo "Boot log" > "$TEST_DATA_DIR/Logs/System/boot.log"

# ============================================================================
# Temp directory
# ============================================================================

echo "Temporary file" > "$TEST_DATA_DIR/Temp/temp1.tmp"
echo "Temporary file" > "$TEST_DATA_DIR/Temp/temp2.tmp"
echo "Temporary file" > "$TEST_DATA_DIR/Temp/temp3.tmp"
echo "Cache data" > "$TEST_DATA_DIR/Temp/cache.tmp"
echo "Swap file" > "$TEST_DATA_DIR/Temp/swap.tmp"

# ============================================================================
# Config directory structure
# ============================================================================

# Config/Fonts
create_ttf "$TEST_DATA_DIR/Config/Fonts/font1.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/font2.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/font3.otf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/InterDisplay-Regular.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/InterDisplay-Bold.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/InterDisplay-Italic.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/InterDisplay-Light.ttf"
create_ttf "$TEST_DATA_DIR/Config/Fonts/InterDisplay-Medium.ttf"

# Config/Themes
echo "Theme config" > "$TEST_DATA_DIR/Config/Themes/theme1.json"
echo "Theme config" > "$TEST_DATA_DIR/Config/Themes/theme2.json"
echo "Theme config" > "$TEST_DATA_DIR/Config/Themes/dark.json"
echo "Theme config" > "$TEST_DATA_DIR/Config/Themes/light.json"

# ============================================================================
# Code directory structure
# ============================================================================

# Code/src
echo "Source code" > "$TEST_DATA_DIR/Code/src/main.rs"
echo "Source code" > "$TEST_DATA_DIR/Code/src/lib.rs"
echo "Source code" > "$TEST_DATA_DIR/Code/src/utils.rs"
echo "Source code" > "$TEST_DATA_DIR/Code/src/main.py"
echo "Source code" > "$TEST_DATA_DIR/Code/src/helper.py"

# Code/tests
echo "Test code" > "$TEST_DATA_DIR/Code/tests/test_main.rs"
echo "Test code" > "$TEST_DATA_DIR/Code/tests/test_utils.rs"
echo "Test code" > "$TEST_DATA_DIR/Code/tests/test_helper.py"

# Code/docs
echo "Documentation" > "$TEST_DATA_DIR/Code/docs/README.md"
echo "Documentation" > "$TEST_DATA_DIR/Code/docs/API.md"
echo "Documentation" > "$TEST_DATA_DIR/Code/docs/GUIDE.md"
create_pdf "$TEST_DATA_DIR/Code/docs/manual.pdf"

echo ""
echo "Test data generation complete!"
echo "Files created in: $TEST_DATA_DIR"
echo ""
echo "Directory structure created:"
echo "  - Photos/ (with Vacation/2024 subdirectories) - Image files"
echo "  - Documents/ (with Projects and Archive subdirectories) - PDFs and text files"
echo "  - Media/ (with Music and Videos subdirectories) - Media files"
echo "  - Backups/ (with 2024/January and 2024/February subdirectories) - Backup files"
echo "  - Logs/ (with Application and System subdirectories) - Log files"
echo "  - Temp/ - Temporary files"
echo "  - Config/ (with Fonts and Themes subdirectories) - Configuration files"
echo "  - Code/ (with src, tests, and docs subdirectories) - Source code files"
echo "  - Root level - Various file types with different naming patterns"
echo ""
echo "Total files generated: ~100+ files across nested directory structure"
echo ""
echo "Usage:"
echo "  Run this script before running tests: ./generate_test_data.sh"
echo "  Or add 'test_data' to .gitignore to exclude generated files from the repository."
