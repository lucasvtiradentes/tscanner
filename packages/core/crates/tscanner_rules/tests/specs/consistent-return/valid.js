function alwaysReturnsValue(x) {
  if (x) {
    return 1;
  }
  return 0;
}

function neverReturnsValue(x) {
  if (x) {
    console.log('x is true');
    return;
  }
  console.log('x is false');
}
