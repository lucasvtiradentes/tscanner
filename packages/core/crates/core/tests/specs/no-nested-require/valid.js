const fs = require('fs');
const path = require('path');
const http = require('http');

function useFs() {
  return fs.readFileSync('file.txt');
}
