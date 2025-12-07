// ============================================================================
// CODE QUALITY RULES
// ============================================================================

// no_console.rs (REGEX)
console.log('debug message');

// no_todo_comments.rs (REGEX)
// TODO: fix this later

// no_empty_function.rs
function emptyFunction() {}

// no_empty_class.rs
class EmptyClass {}

// no_nested_ternary.rs
const nestedTernary = true ? (false ? 1 : 2) : 3;

// no_else_return.rs
function elseAfterReturn(value) {
  if (value) {
    return 1;
  } else {
    return 2;
  }
}

// no_return_await.rs
async function returnAwait() {
  return await Promise.resolve(42);
}

// no_async_without_await.rs
async function asyncWithoutAwait() {
  console.log('no await here');
}

// no_useless_catch.rs
function uselessCatch() {
  try {
    throw new Error('test');
  } catch (e) {
    throw e;
  }
}

// max_params.rs
function tooManyParams(a, b, c, d, e) {
  return a + b + c + d + e;
}

// max_function_length.rs
function longFunction() {
  const a1 = 1;
  const a2 = 2;
  const a3 = 3;
  const a4 = 4;
  const a5 = 5;
  return a1 + a2 + a3 + a4 + a5;
}

// no_unused_vars.rs
const unusedVariable = 'never used';

// ============================================================================
// VARIABLE RULES
// ============================================================================

// no_var.rs
var varVariable = 'should use let or const';

// prefer_const.rs
let neverReassigned = 'should be const';

// no_shadow.rs
const shadowedVar = 'outer';
function shadowingFunction() {
  const shadowedVar = 'inner';
  return shadowedVar;
}

// ============================================================================
// BUG PREVENTION RULES
// ============================================================================

// no_constant_condition.rs
if (true) {
  const x = 1;
}

// no_unreachable_code.rs
function unreachableCode() {
  return 1;
  const neverExecuted = 2;
}

// consistent_return.rs
function inconsistentReturn(value) {
  if (value) {
    return 1;
  }
  return;
}

// ============================================================================
// STYLE RULES
// ============================================================================

// prefer_optional_chain.rs
const obj = {};
const optionalChainCandidate = obj && obj.prop;

// prefer_nullish_coalescing.rs
const nullishValue = null;
const orDefault = nullishValue || 'default';

// ============================================================================
// IMPORT RULES
// ============================================================================

// no_dynamic_import.rs
const dynamicModule = import('./dynamic-module');

// no_default_export.rs
export default function defaultExportFn() {
  return 'default';
}

// no_forwarded_exports.rs
export { something } from './forwarded-module';
export * from './star-export-module';

// no_nested_require.rs
function nestedRequire() {
  const mod = require('nested-module');
  return mod;
}

// no_duplicate_imports.rs
import { foo } from './some-module';
import { bar } from './some-module';

// no_relative_imports.rs
import { util } from './utils';
import { helper } from '../helpers';

// no_absolute_imports.rs
import { abs } from '/absolute/path/module';

// no_alias_imports.rs
import { aliased } from '@alias/module';
