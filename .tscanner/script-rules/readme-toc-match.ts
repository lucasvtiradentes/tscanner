#!/usr/bin/env npx tsx

import { type ScriptIssue, addIssue, runScript } from '../../packages/cli/src/types';

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

function analyzeReadme(file: { path: string; content: string }, issues: ScriptIssue[]) {
  const tocLinks = extractTocLinks(file.content);
  const headings = extractMainHeadings(file.content);

  for (const link of tocLinks) {
    const matchingHeading = headings.find((h) => h.anchor === link.anchor);

    if (!matchingHeading) {
      addIssue(issues, {
        file: file.path,
        line: link.line,
        message: `TOC link "${link.name}" (#${link.anchor}) has no matching heading`,
      });
    }
  }

  for (const heading of headings) {
    const matchingLink = tocLinks.find((l) => l.anchor === heading.anchor);

    if (!matchingLink) {
      addIssue(issues, {
        file: file.path,
        line: heading.line,
        message: `Heading "${heading.text}" is not in TOC (expected: #${heading.anchor})`,
      });
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
      addIssue(issues, {
        file: file.path,
        line: tocLink.line,
        message: `TOC order mismatch: "${tocLink.name}" should come after "${expectedHeading.text}"`,
      });
      break;
    }
  }
}

runScript((input) => {
  const issues: ScriptIssue[] = [];

  const readmeFiles = input.files.filter((f) => f.path.endsWith('README.md') || f.path.endsWith('readme.md'));

  for (const readmeFile of readmeFiles) {
    analyzeReadme(readmeFile, issues);
  }

  return issues;
});
