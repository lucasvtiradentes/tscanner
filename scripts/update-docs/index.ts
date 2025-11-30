import { updateCliUsage } from './update-cli-usage';
import { updateCommands } from './update-commands';
import { updateConfigSection } from './update-config-section';
import { updateContributing } from './update-contributing';
import { updateFeaturesSection } from './update-features-section';
import { updateFooter } from './update-footer';
import { updateImages } from './update-images';
import { updateInspirations } from './update-inspirations';
import { updateMotivation } from './update-motivation';
import { updateQuickStart } from './update-quick-start';
import { updateRules } from './update-rules';
import { updateWaysToUseTscanner } from './update-ways-to-use-tscanner';

type UpdateFn = {
  name: string;
  fn: () => void;
};

const logger = console;

async function main() {
  if (process.env.CI || process.env.GITHUB_ACTIONS) {
    logger.log('[CLI] Skipping local installation in CI environment');
    process.exit(0);
  }

  const updates: UpdateFn[] = [
    { name: 'rules', fn: updateRules },
    { name: 'images', fn: updateImages },
    { name: 'commands', fn: updateCommands },
    { name: 'config-section', fn: updateConfigSection },
    { name: 'quick-start', fn: updateQuickStart },
    { name: 'inspirations', fn: updateInspirations },
    { name: 'contributing', fn: updateContributing },
    { name: 'ways-to-use-tscanner', fn: updateWaysToUseTscanner },
    { name: 'motivation', fn: updateMotivation },
    { name: 'features-section', fn: updateFeaturesSection },
    { name: 'footer', fn: updateFooter },
    { name: 'cli-usage', fn: updateCliUsage },
  ];

  const errors: { name: string; error: string }[] = [];

  for (const update of updates) {
    try {
      update.fn();
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      errors.push({ name: update.name, error: message });
      console.error(`\n✗ ERROR in [${update.name}]:\n  ${message}\n`);
    }
  }

  if (errors.length > 0) {
    console.error('\n========================================');
    console.error(`FAILED: ${errors.length} update(s) had errors:`);
    errors.forEach((e) => console.error(`  - ${e.name}`));
    console.error('========================================\n');
    process.exit(1);
  }

  console.log('\n✓ All updates completed successfully\n');
}

main();
