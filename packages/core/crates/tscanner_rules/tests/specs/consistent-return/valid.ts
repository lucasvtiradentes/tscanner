function alwaysReturnsValue(x: boolean): number {
  if (x) {
    return 1;
  }
  return 0;
}

function neverReturnsValue(x: boolean): void {
  if (x) {
    console.log('x is true');
    return;
  }
  console.log('x is false');
}
