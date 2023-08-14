const semver = require('semver')
const yaml = require('js-yaml');
const fs   = require('fs');
const path = require('path');

module.exports = {
    filesChangedToListOfSubstreamsModified,
    updateVersions,
    listCurrentVersions
}


// given a set of files changed in the repo, it returns the list of substreams 
// that should get their version updated in the manifest.
function filesChangedToListOfSubstreamsModified(changedFiles) {
    const modifiedSubstreams = getAllModifiedSubstreams(changedFiles);
    for (let indirect of getIndirectlyModifiedSubstreams(modifiedSubstreams)) {
        modifiedSubstreams[indirect] = true;
    }
    return Object.keys(modifiedSubstreams);
}

// Returns an array of substream names that have been indirectly modified by a set of changes.
// It is recursive, so it will also include substreams that depend on other substreams that
// have been modified.
function getIndirectlyModifiedSubstreams(directlyModified) {
    const indirectlyModified = [];
    const tmp = Object.keys(directlyModified);
    do {
        let substream = tmp.pop();
        if (!substream) {
            break;
        }

        let deps = getSubstreamDependants(substream);
        for (let dep of deps) {
            if (directlyModified[dep] || indirectlyModified.indexOf(dep) !== -1) {
                continue;
            }

            indirectlyModified.push(dep);
            tmp.push(dep);
        }

    } while (true);

    return indirectlyModified;
}

// returns an object, where each key is the name of a substream that has been directly 
// modified by a set of changes. It can either be modified by modifying its files, or by
// updating a crate from which this substream depends on. The value is just a true boolean.
function getAllModifiedSubstreams(changedFiles) {
    const substreams = {};
    for (let path of changedFiles) {
        const substream = getSubstreamName(path);
        if (substream) {
            substreams[substream] = true;
            continue;
        }

        if (isCrate(path)) {
            const dependants = getCrateDependants(path);
            for (let dependant of dependants) {
                substreams[dependant] = true;
            }
        }
    }
    return substreams;
}

// Will return a yaml object with the substreams manifest for a given substream name.
function loadSubstreamManifest(substreamName) {
    const manifestPath = path.join(getSubstreamDir(substreamName), "substreams.yaml");
    const doc = yaml.load(fs.readFileSync(manifestPath, 'utf8'));
    return doc;
}

// Will save a yaml manifest to the substreams.yaml file for the given substream name.
function writeSubstreamManifest(substreamName, manifest) {
    const manifestPath = path.join(getSubstreamDir(substreamName), "substreams.yaml");
    fs.writeFileSync(manifestPath, yaml.dump(manifest));
}

function updateVersions(substreams, increase) {
    for (let substream of substreams) {
        const manifest = loadSubstreamManifest(substream)
        
        let version = manifest.package.version;
        let newVersion = increaseVersion(version, increase);
        manifest.package.version = newVersion;

        writeSubstreamManifest(substream, manifest);
    }
}

function increaseVersion(oldVersion, increaseType) {
    return "v"+semver.inc(oldVersion, increaseType);
}

// Given a path, relative to the base of the repo, or absolute, it will return the 
// name of the substream on that path. If it doesn't point to a substream project 
// it will return false.
function getSubstreamName(_path) {
    if (path.isAbsolute(_path)) {
        const basePath = path.join(__dirname, "../../");
        _path = path.relative(basePath, _path);
    }

    const substream = _path.split("/")[0];
    const manifest = path.join(getSubstreamDir(substream), "substreams.yaml");
    if (!fs.existsSync(manifest)) {
        return false;
    }

    return substream;
}

// Given a substream name, returns its absolute path.
function getSubstreamDir(substream) {
    return path.join(__dirname, "../../", substream);
}

// Returns true if the project on that path is a rust crate (not a substream).
// It assumes all paths passed to it belong to the substreams repo.
function isCrate(path) {
    const substream = getSubstreamName(path);
    return !substream
}

function getCrateDependants(path) {
    return []; // TODO
}

// For a given substream name, it will return an array of substream names
// that directly depend on it. It will not include dependants that depend on 
// other dependants.
function getSubstreamDependants(substreamName) {
    const dependants = [];
    const allSubstreams = getAllSubstreams();
    for (let substream of allSubstreams) {
        const manifest = loadSubstreamManifest(substream);
        for (let _import in manifest.imports) {
            const importPath = manifest.imports[_import];
            if (importPath.startsWith("http")) {
                continue;
            }

            const relativeToHere = path.join(__dirname, "../../", substream, importPath)
            const dependency = getSubstreamName(relativeToHere);
            if (dependency == substreamName) {
                dependants.push(substream);
            }
        }
    }

    return dependants;
}

// returns all substream names currently in the repo.
function getAllSubstreams() {
    const substreams = [];
    const projects = fs.readdirSync(path.join(__dirname, "../../"));
    for (let project of projects) {
        const name = getSubstreamName(project);
        if (name) {
            substreams.push(name);
        }
    }
    return substreams;
}

function listCurrentVersions() {
    const substreams = getAllSubstreams();
    const versions = {};
    for (let substream of substreams) {
        const manifest = loadSubstreamManifest(substream);
        versions[substream] = manifest.package.version;
    }
    return versions;
}