// ============================================================================
// TYPE SAFETY RULES
// ============================================================================

// no_explicit_any.rs (TS ONLY)
const explicitAny: any = 2;
const asAnyValue = 'test' as any;

// no_implicit_any.rs (TS ONLY)
function implicitAnyParam(param) {
  return param;
}

// no_inferrable_types.rs (TS ONLY)
const inferrableNumber: number = 42;
const inferrableString: string = 'hello';
const inferrableBool: boolean = true;

// no_non_null_assertion.rs (TS ONLY)
const maybeNull: string | null = 'test';
const nonNullAsserted = maybeNull!;

// no_single_or_array_union.rs (TS ONLY)
type SingleOrArray = string | string[];

// no_unnecessary_type_assertion.rs (TS ONLY)
const unnecessaryStringAssertion = 'hello' as string;
const unnecessaryNumberAssertion = 123 as number;
const unnecessaryBoolAssertion = true as boolean;

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

// no_empty_interface.rs (TS ONLY)
interface EmptyInterface{};

// no_nested_ternary.rs
const nestedTernary = true ? (false ? 1 : 2) : 3;

// no_else_return.rs
function elseAfterReturn(value: boolean): number {
  if (value) {
    return 1;
  } else {
    return 2;
  }
}

// no_return_await.rs
async function returnAwait(): Promise<number> {
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
function tooManyParams(a: number, b: number, c: number, d: number, e: number) {
  return a + b + c + d + e;
}

// no_unused_vars.rs
const unusedVariable = 'never used';

// ============================================================================
// VARIABLE RULES
// ============================================================================

// no_var.rs
var varVariable = 'should use let or const';

// prefer_const.rs
const neverReassigned = 'should be const';

// no_shadow.rs
const shadowedVar = 'outer';
function shadowingFunction() {
  const shadowedVar = 'inner';
  return shadowedVar;
}

// ============================================================================
// BUG PREVENTION RULES
// ============================================================================

//no_constant_condition.rs
if (true) {
  console.log('always runs');
}

//no_unreachable_code.rs
function unreachableCode(): number {
  return 1;
  const neverExecuted = 2;
}

//consistent_return.rs
function inconsistentReturn(value: boolean) {
  if (value) {
    return 1;
  }
  return;
}

//no_floating_promises.rs (TS ONLY)
async function fetchData(): Promise<string> {
  return 'data';
}
fetchData();

// ============================================================================
// STYLE RULES
// ============================================================================

// prefer_interface_over_type.rs (TS ONLY)
type ObjectType = {
  name: string;
  age: number;
};

// prefer_type_over_interface.rs (TS ONLY)
interface SomeInterface {
  value: string;
}

// prefer_optional_chain.rs
const obj: { prop?: { nested?: string } } = {};
const optionalChainCandidate = obj && obj.prop;

// prefer_nullish_coalescing.rs
const nullishValue: string | null = null;
const orDefault = nullishValue || 'default';

// imports/no_dynamic_import.rs
const dynamicModule = import('./dynamic-module');

// imports/no_default_export.rs
export default function defaultExportFn() {
  return 'default';
}

// imports/no_forwarded_exports.rs
export { something } from './forwarded-module';
export * from './star-export-module';

// imports/no_nested_require.rs
function nestedRequire() {
  const mod = require('nested-module');
  return mod;
}
