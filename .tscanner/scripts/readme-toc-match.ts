#!/usr/bin/env npx tsx

import { stdin } from 'node:process';

type ScriptFile = {
  path: string;
  content: string;
  lines: string[];
};

type ScriptInput = {
  files: ScriptFile[];
  options?: Record<string, unknown>;
  workspaceRoot: string;
};

type ScriptIssue = {
  file: string;
  line: number;
  column?: number;
  message: string;
};

function addIssue(issues: ScriptIssue[], file: string, line: number, message: string): void {
  issues.push({ file, line, message });
}

function extractTocLinks(content: string): { name: string; anchor: string; line: number }[] {
  const links: { name: string; anchor: string; line: number }[] = [];
  const lines = content.split('\n');

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const tocLinkRegex = /<a href="#([^"]+)">([^<]+)<\/a>/g;

    for (const match of line.matchAll(tocLinkRegex)) {
      const anchor = match[1];
      if (anchor === 'TOC') continue;

      links.push({
        anchor,
        name: match[2].trim(),
        line: i + 1,
      });
    }
  }

  return links;
}

function generateGitHubAnchor(headingText: string): string {
  const hasEmoji = /[\p{Emoji}\u200d]/u.test(headingText);
  const cleaned = headingText
    .toLowerCase()
    .replace(/[^\p{L}\p{N}\s-]/gu, '')
    .trim()
    .replace(/\s+/g, '-');

  return hasEmoji ? `-${cleaned}` : cleaned;
}

function extractMainHeadings(content: string): { text: string; anchor: string; line: number }[] {
  const headings: { text: string; anchor: string; line: number }[] = [];
  const lines = content.split('\n');
  let inCodeBlock = false;

  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];

    if (line.trim().startsWith('```')) {
      inCodeBlock = !inCodeBlock;
      continue;
    }

    if (inCodeBlock) {
      continue;
    }

    const headingMatch = line.match(/^##\s+(.+)/);

    if (headingMatch) {
      const text = headingMatch[1]
        .replace(/<[^>]+>/g, '')
        .replace(/\[([^\]]+)\]\([^)]+\)/g, '$1')
        .trim();

      const anchor = generateGitHubAnchor(text);

      headings.push({
        text,
        anchor,
        line: i + 1,
      });
    }
  }

  return headings;
}

async function main() {
  let data = '';

  for await (const chunk of stdin) {
    data += chunk;
  }

  const input: ScriptInput = JSON.parse(data);
  const issues: ScriptIssue[] = [];

  const readmeFile = input.files.find((f) => f.path.endsWith('README.md') || f.path.endsWith('readme.md'));

  if (!readmeFile) {
    console.log(JSON.stringify({ issues }));
    return;
  }

  const tocLinks = extractTocLinks(readmeFile.content);
  const headings = extractMainHeadings(readmeFile.content);

  for (const link of tocLinks) {
    const matchingHeading = headings.find((h) => h.anchor === link.anchor);

    if (!matchingHeading) {
      addIssue(issues, readmeFile.path, link.line, `TOC link "${link.name}" (#${link.anchor}) has no matching heading`);
    }
  }

  for (const heading of headings) {
    const matchingLink = tocLinks.find((l) => l.anchor === heading.anchor);

    if (!matchingLink) {
      addIssue(
        issues,
        readmeFile.path,
        heading.line,
        `Heading "${heading.text}" is not in TOC (expected: #${heading.anchor})`,
      );
    }
  }

  const tocAnchors = tocLinks.map((l) => l.anchor);
  const headingAnchors = headings.map((h) => h.anchor);
  const commonTocAnchors = tocAnchors.filter((a) => headingAnchors.includes(a));
  const commonHeadingAnchors = headingAnchors.filter((a) => tocAnchors.includes(a));

  for (let i = 0; i < commonTocAnchors.length; i++) {
    if (commonTocAnchors[i] !== commonHeadingAnchors[i]) {
      const tocLink = tocLinks.find((l) => l.anchor === commonTocAnchors[i])!;
      const expectedHeading = headings.find((h) => h.anchor === commonHeadingAnchors[i])!;
      addIssue(
        issues,
        readmeFile.path,
        tocLink.line,
        `TOC order mismatch: "${tocLink.name}" should come after "${expectedHeading.text}"`,
      );
      break;
    }
  }

  console.log(JSON.stringify({ issues }));
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
