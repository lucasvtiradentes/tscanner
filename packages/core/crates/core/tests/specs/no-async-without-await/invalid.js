async function foo() {
  return 1;
}

async function bar() {
  console.log('hello');
}

const baz = async () => {
  return 2;
};

const qux = async () => console.log('world');

async function withPromise() {
  return Promise.resolve(1);
}

class MyClass {
  async method() {
    return 3;
  }
}

const obj = {
  async method() {
    return 4;
  }
};
