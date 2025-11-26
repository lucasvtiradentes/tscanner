function withType(x: number): number {
  return x;
}

const arrowWithType = (x: string): string => x;

const arr = [1, 2, 3];
arr.map((x) => x * 2);
arr.filter((x) => x > 1);
