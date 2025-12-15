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
  Realtime: 'realtime',
  Focus: 'focus',
  Pr: 'pr',
  SpeedCached: 'speed-cached',
  CopyAi: 'copy-ai',
  OneComment: 'one-comment',
  Severity: 'severity',
} as const;

type FeatureId = (typeof FeatureId)[keyof typeof FeatureId];

type FeatureBullet = {
  id: FeatureId;
  title: string;
  description: string;
  packages: Package[];
};

const FEATURE_BULLETS: FeatureBullet[] = [
  {
    id: FeatureId.Rules,
    title: 'Your Rules, Enforced',
    description: `${RULES_COUNT} built-in checks + define your own with regex, scripts, or AI`,
    packages: [Package.Main, Package.Cli, Package.Vscode, Package.Action],
  },
  {
    id: FeatureId.Realtime,
    title: 'See Issues Instantly',
    description: 'Real-time feedback in code editor as you type, no manual scan needed',
    packages: [Package.Main, Package.Vscode],
  },
  {
    id: FeatureId.Focus,
    title: 'Focus on What Matters',
    description: '4 scan modes: whole codebase, branch changes, uncommitted changes or staged changes',
    packages: [Package.Main, Package.Cli, Package.Vscode, Package.Action],
  },
  {
    id: FeatureId.Pr,
    title: 'Catch Before Merge',
    description: 'PR comments show violations with clickable links to exact lines',
    packages: [Package.Main, Package.Action],
  },
  {
    id: FeatureId.SpeedCached,
    title: 'Sub-second Scans',
    description: 'Rust engine processes hundreds of files in <1s, with smart caching',
    packages: [Package.Cli],
  },
  {
    id: FeatureId.CopyAi,
    title: 'Copy for AI',
    description: 'Export issues to clipboard, paste into chat for bulk fixes',
    packages: [Package.Vscode],
  },
  {
    id: FeatureId.OneComment,
    title: 'One Comment, Updated',
    description: 'No spam, same comment updated on each push',
    packages: [Package.Action],
  },
  {
    id: FeatureId.Severity,
    title: 'Not a Blocker',
    description: 'Issues are warnings by default; set as errors to fail CI/lint-staged',
    packages: [Package.Main],
  },
];

const PACKAGE_ORDER: Record<Package, FeatureId[]> = {
  [Package.Main]: [
    FeatureId.Rules,
    FeatureId.Realtime,
    FeatureId.Focus,
    FeatureId.CopyAi,
    FeatureId.Pr,
    FeatureId.Severity,
    FeatureId.SpeedCached,
  ],
  [Package.Cli]: [FeatureId.Rules, FeatureId.SpeedCached, FeatureId.Focus, FeatureId.Severity, FeatureId.SpeedCached],
  [Package.Vscode]: [
    FeatureId.Rules,
    FeatureId.Realtime,
    FeatureId.Focus,
    FeatureId.CopyAi,
    FeatureId.Severity,
    FeatureId.SpeedCached,
  ],
  [Package.Action]: [FeatureId.Rules, FeatureId.Focus, FeatureId.Pr, FeatureId.Severity, FeatureId.OneComment],
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
  const order = PACKAGE_ORDER[pkg];
  const bullets = order
    .map((id) => FEATURE_BULLETS.find((b) => b.id === id))
    .filter((b): b is FeatureBullet => b !== undefined)
    .map(formatBullet)
    .join('\n');

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
