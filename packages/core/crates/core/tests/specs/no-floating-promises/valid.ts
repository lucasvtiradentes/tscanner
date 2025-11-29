await fetch('https://api.example.com/data');

fetch('https://api.example.com/data').catch(handleError);

fetch('https://api.example.com/data').then(onSuccess, onError);

const result = fetch('https://api.example.com/data');

Promise.resolve(42).catch(console.error);

Promise.all([p1, p2]).catch(handleError);

await Promise.race([p1, p2]);

void Promise.resolve();

async function delay(ms: number) {
  await new Promise<void>((resolve) => setTimeout(() => resolve(), ms));
}

await delay(200);

const asyncArrow = async () => {
  return 42;
};

await asyncArrow();

asyncArrow().catch(console.error);

const stored = asyncArrow();

someAsyncFunc();

updateReadmeVersions();

execSync('git add .');
