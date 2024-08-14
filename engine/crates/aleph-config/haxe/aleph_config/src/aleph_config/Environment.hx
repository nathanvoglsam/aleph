package aleph_config;

import aleph_config.BuildType;
import aleph_config.Architecture;
import aleph_config.Platform;

/**
 * Class that encapsulates a view of the game's execution for a config script to read, interpret and
 * make decisions on.
 * 
 * The runtime host is expected to construct this and provide it to the config scripts directly.
 * A user should never have to construct this directly.
 */
@:expose
class Environment {
    /**
     * The platform the game is running on
     */
    public var platform: Platform;

    /**
     * The CPU architecture the game is running on
     */
    public var arch: Architecture;

    /**
     * The build type the game was compiled and is for
     */
    public var buildType: BuildType;

    /**
     * [Description] Internal use only. Constructs a new Environment from the given values.
     * @param target 
     */
    private function new(
        platform: Platform,
        arch: Architecture,
        buildType: BuildType
    ) {
        this.platform = platform;
        this.arch = arch;
        this.buildType = buildType;
    }
    
    /**
     * [Description]
     * Exported static function with a well known name that constructs a new Environment from the
     * raw FFI level types. Internal use only.
     * 
     * This is expected to be exported in the JS module at a well-known location for the Rust glue
     * in aleph-config to find. It is the caller's responsibility to ensure the string IDs for each
     * parameter are valid for the enum types declared here.
     * 
     * @param platform The string ID of the game's build platform
     * @param arch The string ID of the game's build CPU architecture
     * @param buildType The string ID of the build type the game was compiled for
     * @return Environment
     */
    private static function create(
        platform: PlatformId,
        arch: ArchitectureId,
        buildType: BuildTypeId
    ): Environment {
        var platform = new Platform(platform);
        var arch = new Architecture(arch);
        var buildType = new BuildType(buildType);
        return new Environment(platform, arch, buildType);
    }
}