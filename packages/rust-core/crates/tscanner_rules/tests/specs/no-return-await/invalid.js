async function basicReturnAwait() {
  return await fetchData();
}

async function arrowReturnAwait() {
  const fn = async () => {
    return await getData();
  };
}

async function nestedReturnAwait() {
  if (true) {
    return await processData();
  }
}

async function multipleReturns() {
  if (condition) {
    return await fetch();
  }
  return await fallback();
}
