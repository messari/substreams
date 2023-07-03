
const { listCurrentVersions } = require("./versions/versions");


(() => {
    const versions = listCurrentVersions();
    console.log(JSON.stringify(versions));
})();

