var lib = require('./index.js');

var to_hash = "Give me the sha1 hash of this!";
console.log(to_hash);
console.log(lib.sha1(to_hash));

var file_to_hash = "./README.md"
console.log("Hash the README.md");
console.log(lib.sha1File(file_to_hash));

lib.sha1FileAsync(file_to_hash, function(err,resp){
    console.log({
    err,
    resp,
    something_else: "else"});
});
