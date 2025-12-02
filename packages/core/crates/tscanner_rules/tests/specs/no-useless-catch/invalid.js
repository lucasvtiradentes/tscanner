try {
  foo();
} catch (e) {
  throw e;
}

try {
  bar();
} catch (error) {
  throw error;
}

function example() {
  try {
    doSomething();
  } catch (err) {
    throw err;
  }
}

async function asyncExample() {
  try {
    await fetchData();
  } catch (e) {
    throw e;
  }
}
