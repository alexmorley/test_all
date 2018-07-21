var addon = require('../native');

exports.sha1 = addon.sha1;
exports.sha1Async = sha1Async;
exports.sha1File = addon.sha1File;
exports.sha1FileAsync = sha1FileAsync;

function sha1Async(string, callback) {
    callback(null,addon.sha1(string))
}
function sha1FileAsync(filename, callback) {
    callback(null,addon.sha1File(filename))
}
