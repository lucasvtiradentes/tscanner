function test() {
  return 1;
  const unreachable = 2;
}

function throwExample() {
  throw new Error();
  console.log('unreachable');
}

function breakExample() {
  while (true) {
    break;
    doSomething();
  }
}
