fetch('https://api.example.com/data');

Promise.resolve(42);

Promise.reject(new Error('fail'));

Promise.all([p1, p2]);

Promise.race([p1, p2]);

fetch('/api').then(handleSuccess);

Promise.resolve().finally(cleanup);

async function delay(ms: number) {
  await new Promise<void>((resolve) => setTimeout(() => resolve(), ms));
}

delay(200);

const asyncArrow = async () => {
  return 42;
};

asyncArrow();
