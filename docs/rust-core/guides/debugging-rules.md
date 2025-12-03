# Debugging Rules

Practical guide for fixing broken rules and debugging rule issues.

## Common Rule Issues

### False Positives

Rule triggers when it shouldn't.

**Symptoms:**
- Test snapshot shows unexpected issues
- Rule reports violations in valid code
- `invalid.ts` incorrectly marked as having issues

**Common Causes:**
- Visitor matches too broadly (e.g., matching all identifiers instead of specific patterns)
- Missing context checks (e.g., not checking parent node type)
- Incorrect scope tracking in stateful rules

### False Negatives

Rule doesn't trigger when it should.

**Symptoms:**
- `valid.ts` passes but should fail
- Missing issues in snapshot
- Edge cases not caught

**Common Causes:**
- Visitor doesn't match all relevant AST node types
- Missing recursive traversal (`visit_children_with` not called)
- Incomplete pattern matching (e.g., only checking `Pat::Ident`, missing `Pat::Array`)

### Wrong Line/Column Positions

**Symptoms:**
- Diagnostic points to wrong location in snapshot
- Column offset by several characters
- Line number off by one

**Common Causes:**
- Using `span.hi` instead of `span.lo`
- Byte position vs character position confusion
- Not accounting for multi-byte UTF-8 characters

### Missing Edge Cases

**Common Edge Cases:**
- Destructuring patterns (`const [a, b] = arr`)
- Nested scopes (functions within functions)
- Parameter patterns (object/array destructuring in params)
- Type annotations vs runtime code
- JSX/TSX-specific nodes

## Debugging Workflow

### Step 1: Create Minimal Reproduction

Add failing case to test spec:

```bash
# Edit the test file
vim packages/rust-core/crates/core/tests/specs/my-rule/invalid.ts

# Add minimal code that demonstrates the bug
```

Example:
```ts
const x = 1;
function foo(x: number) {
  console.log(x);
}
```

### Step 2: Run Specific Test

```bash
cargo test -p core my_rule::invalid
```

**Expected output:**
```
---- my_rule::invalid stdout ----
Snapshot assertion for 'invalid.ts'
Snapshot does not match:
```

### Step 3: Inspect AST with Debug Print

Add `dbg!()` to rule visitor:

```rust
impl Visit for MyRuleVisitor {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        dbg!(&n);
        dbg!(&n.kind);
        for decl in &n.decls {
            dbg!(&decl.name);
        }
        n.visit_children_with(self);
    }
}
```

Run test again:
```bash
cargo test -p core my_rule::invalid -- --nocapture
```

**Output shows AST structure:**
```
[src/rules/my_rule.rs:42] &n = VarDecl {
    span: Span { lo: BytePos(0), hi: BytePos(11) },
    kind: Const,
    decls: [...]
}
```

### Step 4: Fix Visitor Logic

Based on AST inspection, update visitor to handle the case:

```rust
impl Visit for MyRuleVisitor {
    fn visit_var_decl(&mut self, n: &VarDecl) {
        if matches!(n.kind, VarDeclKind::Let) {
            for decl in &n.decls {
                match &decl.name {
                    Pat::Ident(ident) => {
                        self.check_ident(ident);
                    }
                    Pat::Array(arr) => {
                        for elem in arr.elems.iter().flatten() {
                            self.check_pattern(elem);
                        }
                    }
                    _ => {}
                }
            }
        }
        n.visit_children_with(self);
    }
}
```

### Step 5: Update Snapshots

```bash
cargo test -p core my_rule
cargo insta review
```

**Insta review flow:**
```
- Shows diff between old and new snapshot
- Press 'a' to accept
- Press 'r' to reject
- Press 's' to skip
```

## Inspecting the AST

### Print Entire Node

```rust
dbg!(&node);
```

### Print Specific Fields

```rust
dbg!(&node.span);
dbg!(&node.kind);
```

### Print During Visit

```rust
fn visit_expr(&mut self, n: &Expr) {
    println!("Visiting expr: {:?}", n);
    n.visit_children_with(self);
}
```

### Understanding SWC AST Structure

**Common node types:**

```rust
// Variable declarations
VarDecl {
    kind: VarDeclKind::Let | VarDeclKind::Const | VarDeclKind::Var,
    decls: Vec<VarDeclarator>
}

// Patterns (destructuring, params)
Pat::Ident(BindingIdent)
Pat::Array(ArrayPat)
Pat::Object(ObjectPat)
Pat::Rest(RestPat)
Pat::Assign(AssignPat)

// Type annotations
TsTypeAnn {
    type_ann: Box<TsType>
}

// Functions
FnDecl { function: Function }
Function { params: Vec<Param>, body: Option<BlockStmt> }
ArrowExpr { params: Vec<Pat>, body: BlockStmtOrExpr }
```

### Example: Debugging Pattern Matching

Rule not catching destructured variables:

```rust
fn visit_var_decl(&mut self, n: &VarDecl) {
    for decl in &n.decls {
        dbg!(&decl.name);
    }
}
```

**Output shows:**
```
[src/rules/my_rule.rs:50] &decl.name = Array(
    ArrayPat {
        elems: [Some(Ident(...))]
    }
)
```

**Fix:** Handle `Pat::Array` case.

## Testing Edge Cases

### Add Edge Case to Test Spec

```ts
const destructure = 10;
function test() {
  const [destructure] = [1, 2, 3];
}
```

### Run Test

```bash
cargo test -p core my_rule::invalid
```

### Check Snapshot Diff

```bash
cargo insta review
```

**Expected:**
```diff
+ invalid.ts:3:11 my-rule ━━━━━━━━━━━━━━━━━━━━
+
+   ! Variable 'destructure' shadows a variable in an outer scope.
```

## Fixing Line/Column Issues

### Understanding Spans

```rust
node.span.lo  // Start of node (use this for issue location)
node.span.hi  // End of node
```

### Convert Byte Position to Line/Column

```rust
let (line, col) = get_line_col(self.source, span.lo.0 as usize);
```

**How `get_line_col` works:**
- Iterates through source by character
- Counts newlines for line number
- Counts characters since last newline for column
- Handles multi-byte UTF-8 correctly

### Common Mistakes

**Wrong: Using `span.hi`**
```rust
let (line, col) = get_line_col(self.source, node.span.hi.0 as usize);
```

Points to end of node instead of start.

**Wrong: Using byte index directly**
```rust
let line = source[0..byte_pos].matches('\n').count() + 1;
```

Doesn't handle multi-byte characters.

**Correct:**
```rust
let (line, col) = get_line_col(self.source, node.span.lo.0 as usize);
```

### Example: Fixing Column Offset

**Snapshot shows:**
```
> 3 │   const x = 2;
         ^^^^^
```

Expected column is 10, but getting 5.

**Debug:**
```rust
fn add_variable(&mut self, name: String, span: swc_common::Span) {
    let (line, column) = get_line_col(self.source, span.lo.0 as usize);
    println!("span.lo={}, line={}, col={}", span.lo.0, line, column);
    // ...
}
```

**Fix:** Using wrong span (e.g., keyword span instead of identifier span).

## Regression Prevention

### Always Add Test Case

When fixing a bug, add the failing case to test specs:

```bash
# If it's invalid code (should trigger)
echo "const x = 1; const x = 2;" >> tests/specs/my-rule/invalid.ts

# If it's valid code (should not trigger)
echo "const x = 1; { const x = 2; }" >> tests/specs/my-rule/valid.ts
```

### Run Full Test Suite

```bash
cargo test -p core
cargo insta review
```

### Pre-commit Checklist

1. All tests pass: `cargo test -p core`
2. Snapshots reviewed: `cargo insta review`
3. No debug print statements left in code
4. Edge cases covered in test specs

## Example: Fixing False Positive in `no-shadow`

### Bug Report

Rule incorrectly reports shadowing for catch clause parameters.

**Failing code:**
```ts
try {
  throw new Error();
} catch (err) {
  const err = 'shadowed';
}
```

**Expected:** Should report shadowing (catch param is in scope).

**Actual:** Not being caught.

### Debug Process

**Step 1:** Add to `invalid.ts`
```bash
echo "try { throw new Error(); } catch (err) { const err = 'shadowed'; }" >> tests/specs/no-shadow/invalid.ts
```

**Step 2:** Run test
```bash
cargo test -p core no_shadow::invalid
```

**Step 3:** Inspect AST
```rust
fn visit_catch_clause(&mut self, n: &CatchClause) {
    dbg!("visiting catch clause");
    dbg!(&n.param);
    n.visit_children_with(self);
}
```

**Output:**
```
[src/rules/no_shadow.rs:187] &n.param = Some(Ident(...))
```

**Step 4:** Fix visitor

Missing: Add catch param to scope before visiting body.

```rust
fn visit_catch_clause(&mut self, n: &CatchClause) {
    self.push_scope();
    if let Some(param) = &n.param {
        self.add_param(param);
    }
    n.body.visit_with(self);
    self.pop_scope();
}
```

**Step 5:** Update snapshot
```bash
cargo test -p core no_shadow
cargo insta review
```

**Result:** Snapshot now shows expected shadowing issue.

## Related Documentation

- [Rule System](../02-rule-system.md)
- [Testing](../06-testing.md)
- [SWC AST Explorer](https://astexplorer.net/) - Online AST viewer
