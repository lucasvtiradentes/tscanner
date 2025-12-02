function test1(x: boolean) {
  if (x) {
    return 1;
  } else {
    return 2;
  }
}

function test2(x: boolean) {
  if (x) {
    return true;
  } else {
    console.log('other');
    return false;
  }
}

const test3 = (x: boolean) => {
  if (x) {
    return 'yes';
  } else {
    return 'no';
  }
};

function test4(x: boolean, y: boolean) {
  if (x) {
    return 1;
  } else if (y) {
    return 2;
  } else {
    return 3;
  }
}
