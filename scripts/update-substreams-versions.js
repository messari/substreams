
const { filesChangedToListOfSubstreamsModified, updateVersions } = require("./versions/versions");


(() => {
    let changedFiles;
    let versionType;
    if (process.argv.length < 4) {
        changedFiles = JSON.parse(process.argv[2]);
        versionType = process.argv[3];
    } else {
        changedFiles = process.argv.slice(2, -1);
        versionType = process.argv.at(-1);
    }

    if (!["major", "minor", "patch"].includes(versionType)) {
        throw  ("Version type needs to be major, minor, or patch");
    }

    const modifiedSubstreams = filesChangedToListOfSubstreamsModified(changedFiles);
    updateVersions(modifiedSubstreams, versionType);
})();

