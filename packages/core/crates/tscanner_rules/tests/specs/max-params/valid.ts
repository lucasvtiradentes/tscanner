function noParams() {
  return true;
}

function oneParam(a: string) {
  return a;
}

function twoParams(a: string, b: number) {
  return a;
}

function threeParams(a: string, b: number, c: boolean) {
  return a;
}

function fourParams(a: string, b: number, c: boolean, d: object) {
  return a;
}

const arrowNoParams = () => {
  return true;
};

const arrowOneParam = (a: string) => {
  return a;
};

const arrowTwoParams = (a: string, b: number) => {
  return a;
};

const arrowThreeParams = (a: string, b: number, c: boolean) => {
  return a;
};

const arrowFourParams = (a: string, b: number, c: boolean, d: object) => {
  return a;
};

class MyClass {
  methodNoParams() {
    return true;
  }

  methodOneParam(a: string) {
    return a;
  }

  methodTwoParams(a: string, b: number) {
    return a;
  }

  methodThreeParams(a: string, b: number, c: boolean) {
    return a;
  }

  methodFourParams(a: string, b: number, c: boolean, d: object) {
    return a;
  }
}

export function exportedFourParams(a: string, b: number, c: boolean, d: object) {
  return a;
}
