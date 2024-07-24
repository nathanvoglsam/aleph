package aleph_target;

enum abstract BuildTypeId(String) {
    var Dev = "dev";
    var Retail = "retail";
}

@:expose
class BuildType {
    private var v: BuildTypeId;

    public function new(v: BuildTypeId) {
        this.v = v;
    }

    public function isSame(other: BuildType): Bool {
        return this.v == other.v;
    }

    public function isDev(): Bool {
        return this.isSame(BuildType.DEV);
    }

    public function isRetail(): Bool {
        return this.isSame(BuildType.RETAIL);
    }

    public static var DEV = new BuildType(BuildTypeId.Dev);
    public static var RETAIL = new BuildType(BuildTypeId.Retail);
}
