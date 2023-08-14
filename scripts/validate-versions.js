

(() => {
    const prevVersions = JSON.parse(process.argv[2]);
    const newVersions = JSON.parse(process.argv[3]);
    const modifiedSubstreams = JSON.parse(process.argv[4]);

    const errs = [];
    for (let substream of modifiedSubstreams) {
        if (prevVersions[substream] == newVersions[substream]) {
            errs.push(substream);
        }
    }

    if (errs.length > 0) {
        console.error("The following substreams have not been updated:");
        console.error(errs.join(", "));
        process.exit(1);
    }
})();

