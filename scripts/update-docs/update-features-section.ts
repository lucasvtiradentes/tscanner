import { join, resolve } from 'node:path';
import { DynMarkdown, getJson } from 'markdown-helper';

const rootDir = resolve(__dirname, '..', '..');
const rulesJson: unknown[] = getJson(join(rootDir, 'assets/generated/rules.json'));
const RULES_COUNT = rulesJson.length;

type TFields = 'FEATURES';

const Package = {
  Main: 'main',
  Cli: 'cli',
  Vscode: 'vscode',
  Action: 'action',
} as const;

type Package = (typeof Package)[keyof typeof Package];

const FeatureId = {
  Rules: 'rules',
  Registry: 'registry',
  Focus: 'focus',
  SpeedCached: 'speed-cached',
  Severity: 'severity',
  Realtime: 'realtime',
  CopyAi: 'copy-ai',
  Pr: 'pr',
  OneComment: 'one-comment',
} as const;

type FeatureId = (typeof FeatureId)[keyof typeof FeatureId];

type FeatureBullet = {
  id: FeatureId;
  title: string;
  description: string;
};

const FEATURES: Record<FeatureId, FeatureBullet> = {
  [FeatureId.Rules]: {
    id: FeatureId.Rules,
    title: 'Your Rules, Enforced',
    description: `${RULES_COUNT} built-in checks + define your own with regex, scripts, or AI`,
  },
  [FeatureId.Registry]: {
    id: FeatureId.Registry,
    title: 'Community Rules',
    description: 'Install pre-built rules from registry or share your own with the world',
  },
  [FeatureId.Focus]: {
    id: FeatureId.Focus,
    title: 'Focus on What Matters',
    description: '4 scan modes: whole codebase, branch changes, uncommitted changes or staged changes',
  },
  [FeatureId.SpeedCached]: {
    id: FeatureId.SpeedCached,
    title: 'Sub-second Scans',
    description: 'Rust engine processes hundreds of files in <1s, with smart caching',
  },
  [FeatureId.Severity]: {
    id: FeatureId.Severity,
    title: 'Not a Blocker',
    description: 'Issues are warnings by default; set as errors to fail CI/lint-staged',
  },
  [FeatureId.Realtime]: {
    id: FeatureId.Realtime,
    title: 'See Issues Instantly',
    description: 'Real-time feedback in code editor as you type, no manual scan needed',
  },
  [FeatureId.CopyAi]: {
    id: FeatureId.CopyAi,
    title: 'Copy for AI',
    description: 'Export issues to clipboard, paste into chat for bulk fixes',
  },
  [FeatureId.Pr]: {
    id: FeatureId.Pr,
    title: 'Catch Before Merge',
    description: 'PR comments show violations with clickable links to exact lines',
  },
  [FeatureId.OneComment]: {
    id: FeatureId.OneComment,
    title: 'One Comment, Updated',
    description: 'No spam, same comment updated on each push',
  },
};

const BASE_FEATURES: FeatureId[] = [
  FeatureId.Rules,
  FeatureId.Registry,
  FeatureId.Focus,
  FeatureId.SpeedCached,
  FeatureId.Severity,
];

const UNIQUE_FEATURES: Record<Package, FeatureId[]> = {
  [Package.Main]: [FeatureId.Realtime, FeatureId.CopyAi, FeatureId.Pr, FeatureId.OneComment],
  [Package.Cli]: [],
  [Package.Vscode]: [FeatureId.Realtime, FeatureId.CopyAi],
  [Package.Action]: [FeatureId.Pr, FeatureId.OneComment],
};

const README_PATHS: Record<Package, string> = {
  [Package.Main]: 'README.md',
  [Package.Cli]: 'packages/cli/README.md',
  [Package.Vscode]: 'packages/vscode-extension/README.md',
  [Package.Action]: 'packages/github-action/README.md',
};

function formatBullet(bullet: FeatureBullet): string {
  return `- **${bullet.title}** - ${bullet.description}`;
}

function getFeaturesContent(pkg: Package): string {
  const allFeatures = [...BASE_FEATURES, ...UNIQUE_FEATURES[pkg]];
  const bullets = allFeatures.map((id) => formatBullet(FEATURES[id])).join('\n');

  return `## ⭐ Features<a href="#TOC"><img align="right" src="https://cdn.jsdelivr.net/gh/lucasvtiradentes/tscanner@main/.github/image/up_arrow.png" width="22"></a>

${bullets}`;
}

export function updateFeaturesSection() {
  const packages = Object.values(Package);

  packages.forEach((pkg) => {
    const filePath = README_PATHS[pkg];
    const readme = new DynMarkdown<TFields>(join(rootDir, filePath));
    readme.updateField('FEATURES', getFeaturesContent(pkg));
    readme.saveFile();
  });

  console.log('✓ Updated FEATURES section');
}
