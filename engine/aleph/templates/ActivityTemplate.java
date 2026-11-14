package ${{ANDROID_GAME_PACKAGE}};

import org.libsdl.app.SDLActivity;

/**
 * A sample wrapper class that just calls SDLActivity
 */

public class ${{ANDROID_GAME_NAME}}Activity extends SDLActivity {
    @Override
    protected String[] getLibraries() {
        return new String[] {
            "SDL3",
            "${{ANDROID_GAME_TARGET}}",
            "main"
        };
    }
}