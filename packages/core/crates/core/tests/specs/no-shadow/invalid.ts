const x = 1;
function foo() {
  const x = 2;
}

let y = 'outer';
{
  let y = 'inner';
}

const z = true;
const arrow = () => {
  const z = false;
};

function params(a: number) {
  const a = 5;
}

const destructure = 10;
function test() {
  const [destructure] = [1, 2, 3];
}

const outer = 'value';
function nested() {
  function inner() {
    const outer = 'shadowed';
  }
}

try {
  throw new Error();
} catch (err) {
  const err = 'shadowed';
}
