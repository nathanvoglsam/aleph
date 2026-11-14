#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>

extern "C" int Aleph_main(int argc, char *argv[]);

int main(int argc, char *argv[]) {
    return Aleph_main(argc, argv);
}
