//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::app::{FrameTimer, Keyboard, KeyboardEvent, Mouse, MouseEvent, Window};
use imgui::{ImStr, ImString};
use sdl2::keyboard::Mod;
use std::ffi::CStr;
use std::os::raw::c_void;

// TODO: Windows IME
//   #ifdef _WIN32
//       SDL_SysWMinfo wmInfo;
//       SDL_VERSION(&wmInfo.version);
//       SDL_GetWindowWMInfo(window, &wmInfo);
//       io.ImeWindowHandle = wmInfo.info.win.window;
//   #else
//       (void)window;
//   #endif
//

// TODO: Keyboard input
//   bool ImGui_ImplSDL2_ProcessEvent(const SDL_Event* event)
//   {
//       ImGuiIO& io = ImGui::GetIO();
//       switch (event->type)
//       {
//       case SDL_MOUSEWHEEL:
//           {
//               if (event->wheel.x > 0) io.MouseWheelH += 1;
//               if (event->wheel.x < 0) io.MouseWheelH -= 1;
//               if (event->wheel.y > 0) io.MouseWheel += 1;
//               if (event->wheel.y < 0) io.MouseWheel -= 1;
//               return true;
//           }
//       case SDL_MOUSEBUTTONDOWN:
//           {
//               if (event->button.button == SDL_BUTTON_LEFT) g_MousePressed[0] = true;
//               if (event->button.button == SDL_BUTTON_RIGHT) g_MousePressed[1] = true;
//               if (event->button.button == SDL_BUTTON_MIDDLE) g_MousePressed[2] = true;
//               return true;
//           }
//       case SDL_TEXTINPUT:
//           {
//               io.AddInputCharactersUTF8(event->text.text);
//               return true;
//           }
//       case SDL_KEYDOWN:
//       case SDL_KEYUP:
//           {
//               int key = event->key.keysym.scancode;
//               IM_ASSERT(key >= 0 && key < IM_ARRAYSIZE(io.KeysDown));
//               io.KeysDown[key] = (event->type == SDL_KEYDOWN);
//               io.KeyShift = ((SDL_GetModState() & KMOD_SHIFT) != 0);
//               io.KeyCtrl = ((SDL_GetModState() & KMOD_CTRL) != 0);
//               io.KeyAlt = ((SDL_GetModState() & KMOD_ALT) != 0);
//   #ifdef _WIN32
//               io.KeySuper = false;
//   #else
//               io.KeySuper = ((SDL_GetModState() & KMOD_GUI) != 0);
//   #endif
//               return true;
//           }
//       }
//       return false;
//   }
//

// TODO: Gamepad input?
//   static void ImGui_ImplSDL2_UpdateGamepads()
//   {
//       ImGuiIO& io = ImGui::GetIO();
//       memset(io.NavInputs, 0, sizeof(io.NavInputs));
//       if ((io.ConfigFlags & ImGuiConfigFlags_NavEnableGamepad) == 0)
//           return;
//       // Get gamepad
//       SDL_GameController* game_controller = SDL_GameControllerOpen(0);
//       if (!game_controller)
//       {
//           io.BackendFlags &= ~ImGuiBackendFlags_HasGamepad;
//           return;
//       }
//       // Update gamepad inputs
//       #define MAP_BUTTON(NAV_NO, BUTTON_NO)       { io.NavInputs[NAV_NO] = (SDL_GameControllerGetButton(game_controller, BUTTON_NO) != 0) ? 1.0f : 0.0f; }
//       #define MAP_ANALOG(NAV_NO, AXIS_NO, V0, V1) { float vn = (float)(SDL_GameControllerGetAxis(game_controller, AXIS_NO) - V0) / (float)(V1 - V0); if (vn > 1.0f) vn = 1.0f; if (vn > 0.0f && io.NavInputs[NAV_NO] < vn) io.NavInputs[NAV_NO] = vn; }
//       const int thumb_dead_zone = 8000;           // SDL_gamecontroller.h suggests using this value.
//       MAP_BUTTON(ImGuiNavInput_Activate,      SDL_CONTROLLER_BUTTON_A);               // Cross / A
//       MAP_BUTTON(ImGuiNavInput_Cancel,        SDL_CONTROLLER_BUTTON_B);               // Circle / B
//       MAP_BUTTON(ImGuiNavInput_Menu,          SDL_CONTROLLER_BUTTON_X);               // Square / X
//       MAP_BUTTON(ImGuiNavInput_Input,         SDL_CONTROLLER_BUTTON_Y);               // Triangle / Y
//       MAP_BUTTON(ImGuiNavInput_DpadLeft,      SDL_CONTROLLER_BUTTON_DPAD_LEFT);       // D-Pad Left
//       MAP_BUTTON(ImGuiNavInput_DpadRight,     SDL_CONTROLLER_BUTTON_DPAD_RIGHT);      // D-Pad Right
//       MAP_BUTTON(ImGuiNavInput_DpadUp,        SDL_CONTROLLER_BUTTON_DPAD_UP);         // D-Pad Up
//       MAP_BUTTON(ImGuiNavInput_DpadDown,      SDL_CONTROLLER_BUTTON_DPAD_DOWN);       // D-Pad Down
//       MAP_BUTTON(ImGuiNavInput_FocusPrev,     SDL_CONTROLLER_BUTTON_LEFTSHOULDER);    // L1 / LB
//       MAP_BUTTON(ImGuiNavInput_FocusNext,     SDL_CONTROLLER_BUTTON_RIGHTSHOULDER);   // R1 / RB
//       MAP_BUTTON(ImGuiNavInput_TweakSlow,     SDL_CONTROLLER_BUTTON_LEFTSHOULDER);    // L1 / LB
//       MAP_BUTTON(ImGuiNavInput_TweakFast,     SDL_CONTROLLER_BUTTON_RIGHTSHOULDER);   // R1 / RB
//       MAP_ANALOG(ImGuiNavInput_LStickLeft,    SDL_CONTROLLER_AXIS_LEFTX, -thumb_dead_zone, -32768);
//       MAP_ANALOG(ImGuiNavInput_LStickRight,   SDL_CONTROLLER_AXIS_LEFTX, +thumb_dead_zone, +32767);
//       MAP_ANALOG(ImGuiNavInput_LStickUp,      SDL_CONTROLLER_AXIS_LEFTY, -thumb_dead_zone, -32767);
//       MAP_ANALOG(ImGuiNavInput_LStickDown,    SDL_CONTROLLER_AXIS_LEFTY, +thumb_dead_zone, +32767);
//       io.BackendFlags |= ImGuiBackendFlags_HasGamepad;
//       #undef MAP_BUTTON
//       #undef MAP_ANALOG
//   }

///
///
///
pub struct Imgui {
    context: imgui::Context,
    cursors: Vec<sdl2::mouse::Cursor>,
}

impl Imgui {
    pub fn new() -> Self {
        log::trace!("Initializing ImGui");
        let mut context = imgui::Context::create();
        context.set_ini_filename(None);
        context.set_clipboard_backend(ImguiClipboard::new());
        context.style_mut().child_rounding = 0.0;
        context.style_mut().frame_rounding = 0.0;
        context.style_mut().grab_rounding = 0.0;
        context.style_mut().popup_rounding = 0.0;
        context.style_mut().scrollbar_rounding = 0.0;
        context.style_mut().tab_rounding = 0.0;
        context.style_mut().window_rounding = 0.0;

        let io = context.io_mut();
        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        io.backend_flags |= imgui::BackendFlags::RENDERER_HAS_VTX_OFFSET;

        io.key_map[imgui::Key::Tab as usize] = sdl2::keyboard::Scancode::Tab as u32;
        io.key_map[imgui::Key::LeftArrow as usize] = sdl2::keyboard::Scancode::Left as u32;
        io.key_map[imgui::Key::RightArrow as usize] = sdl2::keyboard::Scancode::Right as u32;
        io.key_map[imgui::Key::UpArrow as usize] = sdl2::keyboard::Scancode::Up as u32;
        io.key_map[imgui::Key::DownArrow as usize] = sdl2::keyboard::Scancode::Down as u32;
        io.key_map[imgui::Key::PageUp as usize] = sdl2::keyboard::Scancode::PageUp as u32;
        io.key_map[imgui::Key::PageDown as usize] = sdl2::keyboard::Scancode::PageDown as u32;
        io.key_map[imgui::Key::Home as usize] = sdl2::keyboard::Scancode::Home as u32;
        io.key_map[imgui::Key::End as usize] = sdl2::keyboard::Scancode::End as u32;
        io.key_map[imgui::Key::Insert as usize] = sdl2::keyboard::Scancode::Insert as u32;
        io.key_map[imgui::Key::Delete as usize] = sdl2::keyboard::Scancode::Delete as u32;
        io.key_map[imgui::Key::Backspace as usize] = sdl2::keyboard::Scancode::Backspace as u32;
        io.key_map[imgui::Key::Space as usize] = sdl2::keyboard::Scancode::Space as u32;
        io.key_map[imgui::Key::Enter as usize] = sdl2::keyboard::Scancode::Return as u32;
        io.key_map[imgui::Key::Escape as usize] = sdl2::keyboard::Scancode::Escape as u32;
        io.key_map[imgui::Key::KeyPadEnter as usize] = sdl2::keyboard::Scancode::KpEnter as u32;
        io.key_map[imgui::Key::A as usize] = sdl2::keyboard::Scancode::A as u32;
        io.key_map[imgui::Key::C as usize] = sdl2::keyboard::Scancode::C as u32;
        io.key_map[imgui::Key::V as usize] = sdl2::keyboard::Scancode::V as u32;
        io.key_map[imgui::Key::X as usize] = sdl2::keyboard::Scancode::X as u32;
        io.key_map[imgui::Key::Y as usize] = sdl2::keyboard::Scancode::Y as u32;
        io.key_map[imgui::Key::Z as usize] = sdl2::keyboard::Scancode::Z as u32;

        let cursors = vec![
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Arrow).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::IBeam).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeAll).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNS).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeWE).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNESW).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::SizeNWSE).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::Hand).unwrap(),
            sdl2::mouse::Cursor::from_system(sdl2::mouse::SystemCursor::No).unwrap(),
        ];

        log::trace!("");
        Self { context, cursors }
    }

    ///
    /// There's some stuff we need to run before we handle mouse events and requests and some other
    /// stuff we need to run after mouse events so we split them into different functions
    ///
    pub fn update_mouse_pos_early(&mut self) {
        let io = self.context.io_mut();

        if io.want_set_mouse_pos {
            crate::app::Mouse::set_pos(io.mouse_pos[0] as i32, io.mouse_pos[1] as i32);
        } else {
            io.mouse_pos[0] = f32::MIN;
            io.mouse_pos[1] = f32::MIN;
        }
    }

    ///
    /// There's some stuff we need to run before we handle mouse events and requests and some other
    /// stuff we need to run after mouse events so we split them into different functions
    ///
    pub fn update_mouse_pos_late(&mut self) {
        let io = self.context.io_mut();

        for event in Mouse::events().iter() {
            match event {
                MouseEvent::MouseWheel(event) => {
                    if event.x > 0 {
                        io.mouse_wheel_h += 1.0
                    };
                    if event.x < 0 {
                        io.mouse_wheel_h -= 1.0
                    };
                    if event.y > 0 {
                        io.mouse_wheel += 1.0
                    };
                    if event.y < 0 {
                        io.mouse_wheel -= 1.0
                    };
                }
                _ => {}
            }
        }

        let mouse_state = Mouse::get_state();
        io.mouse_down[0] = mouse_state.left();
        io.mouse_down[1] = mouse_state.right();
        io.mouse_down[2] = mouse_state.middle();

        if Window::focused() {
            io.mouse_pos = [mouse_state.pos().0 as f32, mouse_state.pos().1 as f32];
        }
    }

    pub fn update_keyboard_input(&mut self) {
        let io = self.context.io_mut();

        for event in Keyboard::events().iter() {
            match event {
                KeyboardEvent::KeyDown(event) => {
                    io.keys_down[event.scancode as usize] = true;
                    io.key_shift = (event.keymod & Mod::LSHIFTMOD).bits() != 0;
                    io.key_ctrl = (event.keymod & Mod::LCTRLMOD).bits() != 0;
                    io.key_alt = (event.keymod & Mod::LALTMOD).bits() != 0;
                }
                KeyboardEvent::KeyUp(event) => {
                    io.keys_down[event.scancode as usize] = false;
                    io.key_shift = (event.keymod & Mod::LSHIFTMOD).bits() != 0;
                    io.key_ctrl = (event.keymod & Mod::LCTRLMOD).bits() != 0;
                    io.key_alt = (event.keymod & Mod::LALTMOD).bits() != 0;
                }
                KeyboardEvent::TextInput(event) => {
                    event.text.chars().for_each(|c| io.add_input_character(c));
                }
            }
        }
    }

    ///
    /// Update the mouse cursor
    ///
    pub fn frame(&mut self, mouse_utils: &sdl2::mouse::MouseUtil) -> imgui::Ui {
        let window_size = Window::size();
        let drawable_size = Window::drawable_size();
        let scale = [
            drawable_size.0 as f32 / window_size.0 as f32,
            drawable_size.1 as f32 / window_size.1 as f32,
        ];

        // TODO: Handle window minimized
        //   // Setup display size (every frame to accommodate for window resizing)
        //   int w, h;
        //   int display_w, display_h;
        //   SDL_GetWindowSize(window, &w, &h);
        //   if (SDL_GetWindowFlags(window) & SDL_WINDOW_MINIMIZED)
        //       w = h = 0;
        //   SDL_GL_GetDrawableSize(window, &display_w, &display_h);
        //   io.DisplaySize = ImVec2((float)w, (float)h);
        //   if (w > 0 && h > 0)
        //       io.DisplayFramebufferScale = ImVec2((float)display_w / w, (float)display_h / h);
        self.context.io_mut().display_size = [window_size.0 as f32, window_size.1 as f32];
        self.context.io_mut().display_framebuffer_scale = scale;

        self.context.io_mut().delta_time = FrameTimer::delta_time() as f32;
        let ui = self.context.frame();

        if (ui.io().config_flags & imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
            == imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE
        {
            return ui;
        }

        let cursor = if let Some(cursor) = ui.mouse_cursor() {
            cursor
        } else {
            return ui;
        };

        if ui.io().mouse_draw_cursor || cursor as i32 == imgui::sys::ImGuiMouseCursor_None {
            mouse_utils.show_cursor(false);
        } else {
            self.cursors[cursor as usize].set();
            mouse_utils.show_cursor(true);
        }

        ui
    }

    pub fn context_mut(&mut self) -> &mut imgui::Context {
        &mut self.context
    }
}

struct ImguiClipboard {}

impl ImguiClipboard {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl imgui::ClipboardBackend for ImguiClipboard {
    fn get(&mut self) -> Option<ImString> {
        unsafe {
            let buf = sdl2::sys::SDL_GetClipboardText();

            if buf.is_null() {
                None
            } else {
                let cstr = CStr::from_ptr(buf as *const _);
                let imstr = ImStr::from_cstr_unchecked(cstr);
                let imstr = imstr.to_owned();
                sdl2::sys::SDL_free(buf as *mut c_void);
                Some(imstr)
            }
        }
    }

    fn set(&mut self, value: &ImStr) {
        unsafe {
            sdl2::sys::SDL_SetClipboardText(value.as_ptr());
        }
    }
}
