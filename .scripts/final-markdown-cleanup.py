#!/usr/bin/env python3
"""
Final Markdown Lint Cleanup
Handles remaining edge cases and special situations
"""

import os
import re
from pathlib import Path

def should_skip_file(filepath: Path) -> bool:
    """Check if file should be skipped"""
    exclude_patterns = ['node_modules', '.gemini', '.git']
    path_str = str(filepath)
    return any(pattern in path_str for pattern in exclude_patterns)

def fix_double_closing_fence(content: str) -> str:
    """Fix cases where ```text appears alone on a line (should be ```)"""
    # Fix standalone ```text or other language specifiers at end of code block
    content = re.sub(r'\n```\w+\s*\n\s*\n', '\n```\n\n', content)
    return content

def fix_inline_html(content: str) -> str:
    """Remove or replace simple inline HTML that can be expressed in markdown"""
    # Replace <T> tags (seems to be an artifact)
    content = re.sub(r'<T>', 'T', content)
    return content

def fix_bare_urls(content: str) -> str:
    """Wrap bare URLs in angle brackets"""
    # Find URLs not already in brackets or parentheses
    # This is a conservative pattern to avoid false positives
    lines = content.split('\n')
    result = []
    in_code_block = False
    
    for line in lines:
        if line.strip().startswith('```'):
            in_code_block = not in_code_block
            result.append(line)
            continue
        
        if not in_code_block:
            # Only fix if URL is standalone or at end of line
            line = re.sub(r'(?<!\(|<)(https?://[^\s<>)\]]+)(?!>|\)|\])', r'<\1>', line)
        
        result.append(line)
    
    return '\n'.join(result)

def fix_emphasis_as_heading(content: str) -> str:
    """Convert emphasis used as heading to HTML comment"""
    # Find lines that are just **text** (emphasis) at the end of sections
    lines = content.split('\n')
    result = []
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        # Check if line is only bold text
        if re.match(r'^\*\*[^*]+\*\*$', stripped):
            # Check if next line is blank or end of file (indicates it's used as heading)
            if i + 1 >= len(lines) or lines[i + 1].strip() == '':
                # Convert to HTML comment
                text = re.sub(r'^\*\*([^*]+)\*\*$', r'<!-- \1 -->', stripped)
                result.append(text)
                continue
        
        result.append(line)
    
    return '\n'.join(result)

def fix_trailing_punctuation_in_headings(content: str) -> str:
    """Remove trailing punctuation from headings (except ?)"""
    lines = content.split('\n')
    result = []
    
    for line in lines:
        # Remove ! at end of heading
        line = re.sub(r'^(#{1,6}\s+.+)!(\s*)$', r'\1\2', line)
        # Remove : at end of heading
        line = re.sub(r'^(#{1,6}\s+.+):(\s*)$', r'\1\2', line)
        result.append(line)
    
    return '\n'.join(result)

def fix_empty_links(content: str) -> str:
    """Fix or remove empty links [text]()"""
    content = re.sub(r'\[([^\]]+)\]\(\)', r'`\1`', content)
    return content

def fix_duplicate_blank_lines(content: str) -> str:
    """Remove excessive blank lines (more than 2 consecutive)"""
    # Replace 3+ blank lines with just 2
    content = re.sub(r'\n\n\n+', '\n\n', content)
    return content

def final_cleanup(filepath: Path) -> bool:
    """Apply final cleanups to a markdown file"""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        
        # Apply all fixes
        content = fix_double_closing_fence(content)
        content = fix_inline_html(content)
        content = fix_emphasis_as_heading(content)
        content = fix_trailing_punctuation_in_headings(content)
        content = fix_empty_links(content)
        content = fix_duplicate_blank_lines(content)
        # Don't auto-fix bare URLs as it can break things
        # content = fix_bare_urls(content)
        
        if content != original_content:
            # Use Windows line endings
            content = content.replace('\n', '\r\n')  if '\r\n' not in content else content
            
            with open(filepath, 'w', encoding='utf-8', newline='') as f:
                f.write(content)
            return True
        
        return False
    
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        return False

def main():
    """Main function"""
    base_path = Path('.')
    md_files = list(base_path.rglob('*.md'))
    md_files = [f for f in md_files if not should_skip_file(f)]
    
    print(f"Found {len(md_files)} markdown files")
    print("")
    
    fixed_count = 0
    for md_file in md_files:
        if final_cleanup(md_file):
            print(f"✓ Fixed: {md_file}")
            fixed_count += 1
    
    print("")
    print(f"Summary: Fixed {fixed_count} of {len(md_files)} files")

if __name__ == '__main__':
    main()
