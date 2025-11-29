import { updateRules } from './update-rules';
import { updateImages } from './update-images';
import { updateCommands } from './update-commands';
import { updateConfigSection } from './update-config-section';
import { updateQuickStart } from './update-quick-start';
import { updateInspirations } from './update-inspirations';
import { updateContributing } from './update-contributing';
import { updateWaysToUseTscanner } from './update-ways-to-use-tscanner';
import { updateMotivation } from './update-motivation';
import { updateFeaturesSection } from './update-features-section';
import { updateFooter } from './update-footer';
import { updateCliUsage } from './update-cli-usage';

type UpdateFn = { name: string; fn: () => void };

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
