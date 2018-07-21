var sha1 = require("rust-sha1");

console.log(sha1.sha1File("package.json"));
console.log(sha1.sha1("package.json"));
