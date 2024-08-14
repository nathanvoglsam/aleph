package aleph_config;

import haxe.DynamicAccess;

/**
 * An abstract over Dynamic that is conventionally expected to be a key-value table that pairs
 * plugin/crate names to their config objects.
 */
abstract ConfigTable(DynamicAccess<Dynamic>) {
    /**
     * [Description] Gets the inner object in its raw, untyped glory
     * @return DynamicAccess<Dynamic>
     */
    inline private function get(name: String): Dynamic {
        var cfg = this.get(name);
        // Make sure the config is there.
        if (cfg != null) {
            return cfg;    
        } else {
            throw new haxe.Exception('Field for \"$name\" not found in config table');
        }
    }
}
