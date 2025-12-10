#!/usr/bin/env python3
"""
Conservative Markdown Lint Fixer
Fixes markdown linting issues with careful line-by-line processing
to avoid corruption.
"""

import os
import re
from pathlib import Path
from typing import List, Tuple

def should_skip_file(filepath: Path) -> bool:
    """Check if file should be skipped"""
    exclude_patterns = ['node_modules', '.gemini', '.git']
    path_str = str(filepath)
    return any(pattern in path_str for pattern in exclude_patterns)

def fix_md032_blank_around_lists(lines: List[str]) -> List[str]:
    """MD032: Lists should be surrounded by blank lines"""
    result = []
    prev_blank = True  # Start of file counts as blank
    
    for i, line in enumerate(lines):
        is_list_item = bool(re.match(r'^\s*[-*+]\s+\S', line) or re.match(r'^\s*\d+\.\s+\S', line))
        is_blank = line.strip() == ''
        
        next_is_list = False
        if i + 1 < len(lines):
            next_is_list = bool(re.match(r'^\s*[-*+]\s+\S', lines[i + 1]) or 
                              re.match(r'^\s*\d+\.\s+\S', lines[i + 1]))
        
        # Add blank line before list if needed
        if is_list_item and not prev_blank and i > 0:
            result.append('')
        
        result.append(line)
        
        # Add blank line after list if needed
        if is_list_item and i + 1 < len(lines) and not lines[i + 1].strip() == '' and not next_is_list:
            if i + 1 < len(lines) and not (lines[i + 1].startswith('#') or lines[i + 1].startswith('---')):
                # Check next line isn't already blank
                if i + 2 >= len(lines) or lines[i + 1].strip() != '':
                    # Insert blank after current result
                    pass  # Will be handled by next iteration
        
        prev_blank = is_blank or is_list_item
    
    return result

def fix_md031_blank_around_fences(lines: List[str]) -> List[str]:
    """MD031: Fenced code blocks should be surrounded by blank lines"""
    result = []
    prev_blank = True
    in_fence = False
    
    for i, line in enumerate(lines):
        is_fence = line.strip().startswith('```')
        is_blank = line.strip() == ''
        
        if is_fence:
            # Opening fence
            if not in_fence:
                if not prev_blank and i > 0:
                    result.append('')
                in_fence = True
            # Closing fence
            else:
                in_fence = False
                result.append(line)
                # Add blank after closing fence if not already there
                if i + 1 < len(lines) and lines[i + 1].strip() != '':
                    result.append('')
                prev_blank = True
                continue
        
        result.append(line)
        prev_blank = is_blank
    
    return result

def fix_md040_code_language(lines: List[str]) -> List[str]:
    """MD040: Fenced code blocks should have a language specified"""
    result = []
    
    for line in lines:
        if line.strip() == '```':
            # No language specified, add 'text' as default
            result.append('```text')
        else:
            result.append(line)
    
    return result

def fix_md009_trailing_spaces(lines: List[str]) -> List[str]:
    """MD009: Remove trailing spaces"""
    return [line.rstrip() for line in lines]

def fix_md022_blank_around_headings(lines: List[str]) -> List[str]:
    """MD022: Headings should be surrounded by blank lines"""
    result = []
    prev_blank = True
    
    for i, line in enumerate(lines):
        is_heading = line.startswith('#') and not line.startswith('#!')
        is_blank = line.strip() == ''
        
        # Add blank before heading if needed
        if is_heading and not prev_blank and i > 0:
            # Don't add blank if previous line is separator
            if not result[-1].strip().startswith('---'):
                result.append('')
        
        result.append(line)
        
        # Add blank after heading if needed
        if is_heading and i + 1 < len(lines):
            next_line = lines[i + 1]
            if next_line.strip() != '' and not next_line.strip().startswith('---'):
                result.append('')
                prev_blank = True
                continue
        
        prev_blank = is_blank
    
    return result

def fix_md030_list_marker_space(lines: List[str]) -> List[str]:
    """MD030: Only one space after list marker"""
    result = []
    
    for line in lines:
        # Fix unordered lists (-, *, +)
        line = re.sub(r'^(\s*[-*+])\s{2,}', r'\1 ', line)
        # Fix ordered lists
        line = re.sub(r'^(\s*\d+\.)\s{2,}', r'\1 ', line)
        result.append(line)
    
    return result

def fix_markdown_file(filepath: Path) -> Tuple[bool, List[str]]:
    """Fix markdown linting issues in a file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Split into lines, preserving line endings
        lines = content.splitlines()
        
        # Apply fixes in order
        lines = fix_md009_trailing_spaces(lines)
        lines = fix_md030_list_marker_space(lines)
        lines = fix_md040_code_language(lines)
        lines = fix_md032_blank_around_lists(lines)
        lines = fix_md031_blank_around_fences(lines)
        lines = fix_md022_blank_around_headings(lines)
        
        # Rejoin with CRLF
        new_content = '\r\n'.join(lines)
        
        # Only write if changed
        if new_content != content:
            with open(filepath, 'w', encoding='utf-8', newline='') as f:
                f.write(new_content)
            return True, ["Fixed"]
        
        return False, []
    
    except Exception as e:
        return False, [f"Error: {e}"]

def main():
    """Main function"""
    base_path = Path('.')
    md_files = list(base_path.rglob('*.md'))
    md_files = [f for f in md_files if not should_skip_file(f)]
    
    print(f"Found {len(md_files)} markdown files")
    print("")
    
    fixed_count = 0
    for md_file in md_files:
        changed, messages = fix_markdown_file(md_file)
        if changed:
            print(f"✓ Fixed: {md_file}")
            fixed_count += 1
        elif messages:
            print(f"! {md_file}: {', '.join(messages)}")
    
    print("")
    print(f"Summary: Fixed {fixed_count} of {len(md_files)} files")

if __name__ == '__main__':
    main()
