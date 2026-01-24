## Run from project root: pwsh -ExecutionPolicy Bypass -File ./3rdparty/build.ps1
##
## Building tracy also needs this on Fedora:
## sudo dnf install libxkbcommon-devel wayland-devel wayland-protocols-devel libglvnd-devel dbus-devel

$StartDirectory = Resolve-Path ./

# Create Needed Folders
New-Item ./.aleph/deps/install -ItemType Directory -Force
New-Item ./.aleph/deps/build -ItemType Directory -Force

# Capture Directories
$InstallDirectory = Resolve-Path ./.aleph/deps/install
$BuildDirectory = Resolve-Path ./.aleph/deps/build
$ThirdPartyDirectory = Resolve-Path ./3rdparty

# Move to the Build
Set-Location $BuildDirectory
cmake -G "Ninja" `
    -DCMAKE_INSTALL_PREFIX="$InstallDirectory" `
    -DCMAKE_BUILD_TYPE="Release" `
    -DCMAKE_POSITION_INDEPENDENT_CODE=TRUE `
    -DCMAKE_OSX_DEPLOYMENT_TARGET=15.0 `
    $ThirdPartyDirectory

cmake --build ./ --config Release
cmake --install ./ --config Release

Set-Location $StartDirectory
