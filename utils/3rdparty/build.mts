/// <reference types="@types/node"/>

// Run from the project root: aleph node ./utils/3rdparty/build.mts
//
// Building tracy also needs this on Fedora:
// sudo dnf install libxkbcommon-devel wayland-devel wayland-protocols-devel libglvnd-devel dbus-devel

import * as fs from "node:fs";
import * as path from "node:path";
import {parseArgs} from "node:util";
import {spawnSync} from "node:child_process";

const {values} = parseArgs({
    options: {
        "debug-info": {type: "boolean"},
        "build-root": {type: "string"},
        "cmake-dir": {type: "string"},
        "no-tracy": {type: "boolean"},
    },
});

function run_cmake(cwd: string, args: string[]) {
    const result = spawnSync("cmake", args, {
        cwd,
        stdio: "inherit",
    });

    if (result.error) {
        throw result.error;
    }

    if (result.status !== 0) {
        throw new Error("cmake invocation failed");
    }
}

function main(): void {
    const projectDirectory = process.cwd();

    // Either resolves to the directory referenced by the ${build-root}/.aleph/deps parameter, if
    // present, or it will resolve to ${cwd}/.aleph/deps.
    const depsDirectory = path.join(values["build-root"] ?? projectDirectory, ".aleph", "deps");

    // Build for "Release" by default, but sometimes you want debug symbols for profiling.
    const buildType = values["debug-info"] ? "RelWithDebInfo" : "Release";

    const noTracy = values["no-tracy"] ? "ON" : "OFF";

    // These are always placed in the 'deps' directory, where ever that has been configured
    const installDirectory = path.resolve(path.join(depsDirectory, "install"));
    const buildDirectory = path.resolve(path.join(depsDirectory, "build"));

    // However, we do need to find the CMakeLists.txt. We still assume we're running from the root
    // of an aleph workspace!
    const listDir = values["cmake-dir"] ?? path.join(projectDirectory, "utils", "3rdparty");

    fs.mkdirSync(installDirectory, {recursive: true});
    fs.mkdirSync(buildDirectory, {recursive: true});

    const args = [
        "-G",
        "Ninja",
        `-DALEPH_NO_TRACY=${noTracy}`,
        `-DCMAKE_INSTALL_PREFIX=${installDirectory}`,
        `-DCMAKE_BUILD_TYPE=${buildType}`,
        "-DCMAKE_POSITION_INDEPENDENT_CODE=TRUE",
        "-DCMAKE_OSX_DEPLOYMENT_TARGET=15.0",
        "-DCOMPILER_SUPPORTS_Werror=FALSE",
        listDir,
    ];
    run_cmake(buildDirectory, args);
    run_cmake(buildDirectory, ["--build", "./", "--config", buildType, "--parallel"]);
    run_cmake(buildDirectory, ["--install", "./", "--config", buildType]);
}

main();
