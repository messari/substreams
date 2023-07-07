
const { filesChangedToListOfSubstreamsModified } = require("./versions/versions");


(() => {
    let changedFiles;
    if (process.argv.length < 3) {
        changedFiles = JSON.parse(process.argv[2]);
    } else {
        changedFiles = process.argv.slice(2, -1);
    }

    const modifiedSubstreams = filesChangedToListOfSubstreamsModified(changedFiles);
    console.log(JSON.stringify(modifiedSubstreams));
})();

