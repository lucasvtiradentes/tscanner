function noType(x) {
  return x;
}

const arrowNoType = (x) => x;

function destructuredNoType({ a, b }) {
  return a + b;
}

function restNoType(...args) {
  return args;
}
