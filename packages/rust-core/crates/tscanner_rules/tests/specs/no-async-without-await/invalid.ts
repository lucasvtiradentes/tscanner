async function foo() {
  const x = 1;
}

async function bar() {
  console.log('hello');
}

const baz = async () => {
  const x = 2;
};

const qux = async () => console.log('world');

class MyClass {
  async method() {
    const x = 3;
  }
}

const obj = {
  async method() {
    const x = 4;
  }
};
