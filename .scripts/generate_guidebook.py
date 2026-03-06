#!/usr/bin/env python3
"""
Fusion Programming Language - Complete Guidebook Generator
Generates a comprehensive 100+ page guidebook from all documentation sources
"""

import re
from pathlib import Path

# Base directory
BASE_DIR = Path(__file__).parent.parent

# Source files to combine
SOURCES = {
    "intro": BASE_DIR / "README.md",
    "quick_start": BASE_DIR / "QuickStartGuide.md",
    "user_guide": BASE_DIR / "docs/guides/User_Guide.md",
    "developer_guide": BASE_DIR / "docs/guides/Developer_Guide.md",
    "product_guide": BASE_DIR / "docs/guides/Product_Guide.md",
    "technical_sheet": BASE_DIR / "docs/guides/Technical_Sheet.md",
    "collections_guide": BASE_DIR / "docs/guides/Collections_Complete_Guide.md",
    "type_system": BASE_DIR / "docs/design/Core_Type_System_Design.md",
    "roadmap": BASE_DIR / "docs/roadmap/FUSION_v0.2.0_ROADMAP.md",
    "changelog": BASE_DIR / "ChangeLog.md",
}

# Book chapters (docs/book)
BOOK_DIR = BASE_DIR / "docs/book"

# Text documentation from Files/Docs
TEXT_DOCS = [
    BASE_DIR / "Files/Docs/Fusion Programming.txt",
    BASE_DIR
    / "Files/Docs/Fusion Developer's Guide_ Setup, Internals, and Contribution (v1.0).txt",
    BASE_DIR
    / "Files/Docs/Fusion Programming Language_ The Complete Developer Manual (v1.0).txt",
    BASE_DIR
    / "Files/Docs/Analysis of Missing Core Library Categories in Fusion Design.txt",
    BASE_DIR / "Files/Docs/FIPS 140-2 Security Policy.txt",
]

# Examples to include
EXAMPLES = {
    "test_all": BASE_DIR / "test_all.fu",
    "test_collections": BASE_DIR / "test_collections_complete.fu",
    "test_string": BASE_DIR / "test_string_cast.fu",
    "test_borrow": BASE_DIR / "test_borrow.fu",
}

OUTPUT = BASE_DIR / "docs/guides/FUSION_COMPLETE_GUIDEBOOK.md"


def read_file(filepath):
    """Read file content safely"""
    try:
        with open(filepath, "r", encoding="utf-8", errors="ignore") as f:
            return f.read()
    except Exception as e:
        print(f"Warning: Could not read {filepath}: {e}")
        return ""


def clean_markdown(text: str) -> str:
    """Normalize markdown to reduce markdownlint warnings."""
    lines = text.splitlines()

    # Demote multiple H1 headings to H2 after the first.
    h1_seen = False
    for i, line in enumerate(lines):
        if line.startswith("# "):
            if h1_seen:
                lines[i] = "## " + line[2:]
            else:
                h1_seen = True

    text = "\n".join(lines)

    # Replace inline HTML blocks with markdown-friendly equivalents.
    text = re.sub(r"<div[^>]*>", "", text, flags=re.IGNORECASE)
    text = re.sub(r"</div>", "", text, flags=re.IGNORECASE)
    text = re.sub(r"<br\s*/?>", "", text, flags=re.IGNORECASE)
    text = re.sub(r"<hr\s*/?>", "\n---\n", text, flags=re.IGNORECASE)

    def img_repl(match: re.Match) -> str:
        src = match.group(1) or ""
        alt = match.group(2) or ""
        if not src:
            return ""
        return f"![{alt}]({src})"

    text = re.sub(
        r"<img[^>]*src=\"([^\"]+)\"[^>]*alt=\"([^\"]*)\"[^>]*>",
        img_repl,
        text,
        flags=re.IGNORECASE,
    )
    text = re.sub(
        r"<img[^>]*alt=\"([^\"]*)\"[^>]*src=\"([^\"]+)\"[^>]*>",
        lambda m: f"![{m.group(1)}]({m.group(2)})",
        text,
        flags=re.IGNORECASE,
    )

    # Normalize unordered list markers to '- '.
    text = re.sub(r"^(\s*)\*\s+", r"\1- ", text, flags=re.MULTILINE)

    # Normalize ordered list numbering to '1.'.
    def ol_repl(match: re.Match) -> str:
        indent = match.group(1)
        return f"{indent}1."

    text = re.sub(r"^(\s*)\d+\.", ol_repl, text, flags=re.MULTILINE)

    # Remove trailing punctuation from headings.
    def heading_repl(match: re.Match) -> str:
        hashes = match.group(1)
        title = match.group(2).rstrip()
        title = re.sub(r"[.:;!?]+$", "", title).rstrip()
        return f"{hashes} {title}"

    text = re.sub(r"^(#{1,6})\s+(.*)$", heading_repl, text, flags=re.MULTILINE)

    # Ensure blank lines around lists.
    lines = text.splitlines()
    out = []
    list_re = re.compile(r"^\s*(?:-|\d+\.)\s+")
    for i, line in enumerate(lines):
        is_list = bool(list_re.match(line))
        prev = out[-1] if out else ""
        if is_list and prev.strip() != "" and not list_re.match(prev):
            out.append("")
        out.append(line)
        next_line = lines[i + 1] if i + 1 < len(lines) else ""
        if is_list and next_line.strip() == "":
            # keep single blank line; handled below
            pass
        elif is_list and not list_re.match(next_line) and next_line.strip() != "":
            out.append("")

    # Collapse multiple blank lines to a single blank line.
    cleaned = []
    blank = False
    for line in out:
        if line.strip() == "":
            if not blank:
                cleaned.append("")
            blank = True
        else:
            cleaned.append(line)
            blank = False

    return "\n".join(cleaned).strip() + "\n"


def generate_toc():
    """Generate comprehensive table of contents"""
    return """<!-- markdownlint-disable MD024 -->
# The Complete Fusion Programming Language Guidebook

**Version**: 1.0.0  
**Date**: December 2025  
**Status**: Production Ready  
**Publisher**: Quantum Secure Technologies Inc.

---

## 📘 About This Guidebook

This comprehensive guidebook combines all official Fusion documentation, tutorials, examples, and design specifications into a single authoritative reference. Whether you're a beginner or an experienced developer, this guide will take you from basic concepts to advanced features including quantum computing and AI/ML integration.

**What You'll Learn**:
- Complete language syntax and semantics
- Memory safety with the borrow checker
- Building production applications
- Quantum-ready cryptography
- Machine learning and GPU acceleration
- WebAssembly deployment
- Advanced type system features
- Best practices and design patterns

---

## 📚 Table of Contents

### Part I: Introduction & Getting Started
1. [Welcome to Fusion](#part-i-welcome-to-fusion)
2. [Installation and Setup](#installation-and-setup)
3. [Quick Start Guide](#quick-start-guide)
4. [Your First Program](#your-first-program)

### Part II: Language Fundamentals
5. [Syntax and Structure](#syntax-and-structure)
6. [Variables and Types](#variables-and-types)
7. [Control Flow](#control-flow)
8. [Functions](#functions)
9. [Classes and OOP](#classes-and-oop)
10. [Modules and Packages](#modules-and-packages)

### Part III: Advanced Language Features
11. [Generics and Traits](#generics-and-traits)
12. [Pattern Matching](#pattern-matching)
13. [Error Handling](#error-handling)
14. [Closures and Higher-Order Functions](#closures-and-higher-order-functions)

### Part IV: Memory Management & Safety
15. [Understanding Memory Safety](#understanding-memory-safety)
16. [The Borrow Checker](#the-borrow-checker)
17. [Ownership and Lifetimes](#ownership-and-lifetimes)
18. [Garbage Collection Mode](#garbage-collection-mode)

### Part V: Standard Library
19. [Collections (Vector, HashMap, HashSet)](#collections)
20. [String Processing](#string-processing)
21. [Option and Result Types](#option-and-result-types)
22. [File I/O](#file-io)
23. [Iterator Patterns](#iterator-patterns)

### Part VI: Security & Cryptography
24. [Hybrid Cryptography System](#hybrid-cryptography-system)
25. [Post-Quantum Cryptography](#post-quantum-cryptography)
26. [Zero-Knowledge Proofs](#zero-knowledge-proofs)
27. [Secure Coding Practices](#secure-coding-practices)

### Part VII: AI/ML & GPU Computing
28. [Tensor Operations](#tensor-operations)
29. [Neural Networks](#neural-networks)
30. [GPU Acceleration](#gpu-acceleration)
31. [Model Deployment](#model-deployment)

### Part VIII: Quantum Computing
32. [Quantum Circuits](#quantum-circuits)
33. [Quantum Algorithms](#quantum-algorithms)
34. [Hybrid Classical-Quantum Programming](#hybrid-classical-quantum-programming)

### Part IX: Tools & Development
35. [Build System](#build-system)
36. [Package Manager](#package-manager)
37. [LSP and IDE Integration](#lsp-and-ide-integration)
38. [Testing Framework](#testing-framework)
39. [Debugging and Profiling](#debugging-and-profiling)

### Part X: Advanced Topics
40. [WebAssembly Deployment](#webassembly-deployment)
41. [Multi-File Projects](#multi-file-projects)
42. [FFI and Unsafe Code](#ffi-and-unsafe-code)
43. [Compiler Internals](#compiler-internals)
44. [Performance Optimization](#performance-optimization)

### Part XI: Real-World Applications
45. [Web Applications](#web-applications)
46. [System Programming](#system-programming)
47. [Blockchain Applications](#blockchain-applications)
48. [Embedded Systems](#embedded-systems)

### Appendices
- [Appendix A: Complete Language Reference](#appendix-a-language-reference)
- [Appendix B: Standard Library API](#appendix-b-standard-library-api)
- [Appendix C: Compiler Flags and Options](#appendix-c-compiler-flags)
- [Appendix D: Migration Guides](#appendix-d-migration-guides)
- [Appendix E: v0.2.0 Roadmap](#appendix-e-roadmap)
- [Appendix F: Example Programs](#appendix-f-examples)
- [Appendix G: Glossary](#appendix-g-glossary)

---
"""


def main():
    """Generate the complete guidebook"""
    print("Generating Fusion Complete Guidebook...")

    content = []

    # Add TOC
    content.append(generate_toc())

    # Part I: Introduction
    content.append("\n\n# Part I: Welcome to Fusion\n\n")
    content.append("## Overview\n\n")
    if SOURCES["intro"].exists():
        content.append(read_file(SOURCES["intro"]))

    # Part II: Getting Started
    content.append("\n\n# Part II: Getting Started\n\n")
    content.append("## Installation and Setup\n\n")
    if SOURCES["quick_start"].exists():
        content.append(read_file(SOURCES["quick_start"]))

    # Part III: User Guide
    content.append("\n\n# Part III: Language Fundamentals\n\n")
    if SOURCES["user_guide"].exists():
        content.append(read_file(SOURCES["user_guide"]))

    # Part III.b: Book (detailed chapters)
    if BOOK_DIR.exists():
        content.append("\n\n# Part III-B: Fusion Book (Detailed Chapters)\n\n")
        readme = BOOK_DIR / "README.md"
        if readme.exists():
            content.append(read_file(readme))
        chapters = sorted([p for p in BOOK_DIR.glob("chapter-*.md")])
        for chapter in chapters:
            content.append("\n\n")
            content.append(read_file(chapter))
        appendices = sorted([p for p in BOOK_DIR.glob("appendix-*.md")])
        if appendices:
            content.append("\n\n# Book Appendices\n\n")
            for appendix in appendices:
                content.append("\n\n")
                content.append(read_file(appendix))

    # Part IV: Complete Programming Guide (from text file)
    content.append("\n\n# Part IV: Complete Language Tutorial\n\n")
    content.append("## Comprehensive Programming Guide\n\n")
    if TEXT_DOCS[0].exists():
        content.append(read_file(TEXT_DOCS[0]))

    # Part V: Developer Guide
    content.append("\n\n# Part V: Developer Guide & Internals\n\n")
    if SOURCES["developer_guide"].exists():
        content.append(read_file(SOURCES["developer_guide"]))
    if TEXT_DOCS[1].exists():
        content.append("\n\n## Extended Developer Documentation\n\n")
        content.append(read_file(TEXT_DOCS[1]))

    # Part VI: Collections
    content.append("\n\n# Part VI: Collections and Data Structures\n\n")
    if SOURCES["collections_guide"].exists():
        content.append(read_file(SOURCES["collections_guide"]))

    # Part VII: Type System
    content.append("\n\n# Part VII: Advanced Type System\n\n")
    if SOURCES["type_system"].exists():
        content.append(read_file(SOURCES["type_system"]))

    # Part VIII: Security
    content.append("\n\n# Part VIII: Security and Cryptography\n\n")
    content.append("## FIPS 140-2 Compliance\n\n")
    if TEXT_DOCS[3].exists():
        content.append(read_file(TEXT_DOCS[3]))

    # Part IX: Technical Specifications
    content.append("\n\n# Part IX: Technical Specifications\n\n")
    if SOURCES["technical_sheet"].exists():
        content.append(read_file(SOURCES["technical_sheet"]))
    if SOURCES["product_guide"].exists():
        content.append("\n\n## Product Overview\n\n")
        content.append(read_file(SOURCES["product_guide"]))

    # Part X: Examples
    content.append("\n\n# Part X: Complete Code Examples\n\n")
    for name, path in EXAMPLES.items():
        if path.exists():
            content.append(f"\n\n## Example: {name}\n\n")
            content.append(f"```fusion\n{read_file(path)}\n```\n")

    # Part XI: Roadmap
    content.append("\n\n# Part XI: Future Development\n\n")
    if SOURCES["roadmap"].exists():
        content.append(read_file(SOURCES["roadmap"]))

    # Part XII: Changelog
    content.append("\n\n# Part XII: Project History\n\n")
    if SOURCES["changelog"].exists():
        content.append(read_file(SOURCES["changelog"]))

    # Appendices - Additional documentation
    content.append("\n\n# Appendices\n\n")
    content.append("## Appendix A: Additional Documentation\n\n")
    if TEXT_DOCS[2].exists():
        content.append(read_file(TEXT_DOCS[2]))

    # Write output
    full_content = "".join(content)
    full_content = clean_markdown(full_content)
    OUTPUT.parent.mkdir(parents=True, exist_ok=True)

    with open(OUTPUT, "w", encoding="utf-8") as f:
        f.write(full_content)

    # Statistics
    lines = full_content.count("\n")
    words = len(full_content.split())
    chars = len(full_content)
    pages = lines // 50  # Approximate pages (50 lines per page)

    print("\n✅ Guidebook generated successfully!")
    print(f"📄 Output: {OUTPUT}")
    print("📊 Statistics:")
    print(f"   - Lines: {lines:,}")
    print(f"   - Words: {words:,}")
    print(f"   - Characters: {chars:,}")
    print(f"   - Estimated Pages: {pages}")
    print("\n🎉 Complete!")


if __name__ == "__main__":
    main()
