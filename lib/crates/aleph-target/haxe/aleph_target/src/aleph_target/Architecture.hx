package aleph_target;

enum abstract ArchitectureId(String) {
    var X8664 = "x86_64";
    var AArch64 = "aarch64";
}

@:expose
class Architecture {
    private var v: ArchitectureId;

    public function new(v: ArchitectureId) {
        this.v = v;
    }

    public function isSame(other: Architecture): Bool {
        return this.v == other.v;
    }

    public function isX8664(): Bool {
        return this.isSame(Architecture.X8664);
    }

    public function isAArch64(): Bool {
        return this.isSame(Architecture.AARCH64);
    }

    public static var X8664 = new Architecture(ArchitectureId.X8664);
    public static var AARCH64 = new Architecture(ArchitectureId.AArch64);
}
