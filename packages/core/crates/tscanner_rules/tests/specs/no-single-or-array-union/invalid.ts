type Foo = string | string[];

type Bar = number | number[];

function baz(x: boolean | boolean[]) {}

interface Props {
  value: bigint | bigint[];
}

const handler = (data: symbol | symbol[]): void => {};

type Mixed = object | object[];
