function inconsistent(x: boolean) {
  if (x) {
    return 1;
  }
  return;
}

const arrowInconsistent = (x: boolean) => {
  if (x) {
    return 1;
  }
  return;
};
