const x = 1;
function foo() {
  const y = 2;
}

let a = 'outer';
{
  let b = 'inner';
}

const z = true;
const arrow = () => {
  const w = false;
};

function params(p: number) {
  const q = 5;
}

const original = 10;
function test() {
  const [different] = [1, 2, 3];
}

function sibling1() {
  const same = 'value';
}
function sibling2() {
  const same = 'value';
}

try {
  throw new Error();
} catch (err) {
  console.log(err);
}
