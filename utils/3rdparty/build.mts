/// <reference types="@types/node"/>

// Run from the project root: aleph node ./utils/3rdparty/build.mts
//
// Building tracy also needs this on Fedora:
// sudo dnf install libxkbcommon-devel wayland-devel wayland-protocols-devel libglvnd-devel dbus-devel

import * as fs from "node:fs";
import * as path from "node:path";
import {spawnSync} from "node:child_process";

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
    const toolsDirectory = path.join(projectDirectory, "local", "tools");
    const installDirectory = path.resolve(path.join(toolsDirectory, "install"));
    const buildDirectory = path.resolve(path.join(toolsDirectory, "build"));
    const listDir = path.join(projectDirectory, "utils", "3rdparty");

    fs.mkdirSync(installDirectory, {recursive: true});
    fs.mkdirSync(buildDirectory, {recursive: true});

    const args = [
        "-G",
        "Ninja",
        `-DCMAKE_INSTALL_PREFIX=${installDirectory}`,
        `-DCMAKE_BUILD_TYPE=Release`,
        "-DCMAKE_OSX_DEPLOYMENT_TARGET=15.0",
        listDir,
    ];
    run_cmake(buildDirectory, args);
    run_cmake(buildDirectory, ["--build", "./", "--config", "Release", "--parallel"]);
    run_cmake(buildDirectory, ["--install", "./", "--config", "Release"]);
}

main();
