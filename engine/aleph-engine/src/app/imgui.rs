//
//
// This file is a part of Aleph
//
// <ALEPH_REPO_REPLACE>
//
// <ALEPH_LICENSE_REPLACE>
//

use crate::platform::clipboard::Clipboard;
use crate::platform::frame_timer::FrameTimer;
use crate::platform::keyboard::{Keyboard, KeyboardEvent, Mod, Scancode};
use crate::platform::mouse::{Cursor, Mouse, MouseEvent};
use crate::platform::window::Window;
use render::imgui::{ImStr, ImString, StyleColor};
use palette::{IntoColor, Srgb};

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
pub struct ImguiStyleBuilder {
    background_colour: Srgb,
    separator_colour: Srgb,
    title_colour: Srgb,
    title_unfocused: Srgb,
    slider_colour: Srgb,
    scrollbar_colour: Srgb,
    frame_colour: Srgb,
    border_colour: Srgb,
    button_colour: Srgb,
    checkmark_colour: Srgb,
    header_colour: Srgb,
    resize_grip_colour: Srgb,
    tab_colour: Srgb,
    light_theme: bool,
}

type Hsv = palette::Hsv<palette::encoding::Srgb, f32>;

impl ImguiStyleBuilder {
    ///
    /// Create a new builder with a default light colour scheme
    ///
    pub fn light() -> Self {
        let builder = Self {
            background_colour: Default::default(),
            separator_colour: Default::default(),
            title_colour: Default::default(),
            title_unfocused: Default::default(),
            slider_colour: Default::default(),
            scrollbar_colour: Default::default(),
            frame_colour: Default::default(),
            border_colour: Default::default(),
            button_colour: Default::default(),
            checkmark_colour: Default::default(),
            header_colour: Default::default(),
            resize_grip_colour: Default::default(),
            tab_colour: Default::default(),
            light_theme: true,
        };

        builder
            .primary([0.0 / 255.0, 215.0 / 255.0, 255.0 / 255.0])
            .background([0.96, 0.96, 0.96])
    }

    ///
    /// Create a new builder with a default dark colour scheme
    ///
    pub fn dark() -> Self {
        let builder = Self {
            background_colour: Default::default(),
            separator_colour: Default::default(),
            title_colour: Default::default(),
            title_unfocused: Default::default(),
            slider_colour: Default::default(),
            scrollbar_colour: Default::default(),
            frame_colour: Default::default(),
            border_colour: Default::default(),
            button_colour: Default::default(),
            checkmark_colour: Default::default(),
            header_colour: Default::default(),
            resize_grip_colour: Default::default(),
            tab_colour: Default::default(),
            light_theme: false,
        };

        builder
            .primary([132.0 / 255.0, 61.0 / 255.0, 146.0 / 255.0])
            .background([0.1, 0.1, 0.1])
    }

    ///
    ///
    ///
    pub fn primary(mut self, colour: [f32; 3]) -> Self {
        let colour: Hsv = Srgb::new(colour[0], colour[1], colour[2]).into_hsv();

        self.separator_colour = Srgb::from(colour);
        self.title_colour = Srgb::from(Self::value_shift_mul(colour, 0.5, self.light_theme));
        self.slider_colour = Srgb::from(colour);
        self.button_colour = Srgb::from(Self::value_shift_mul(colour, 0.9, false));
        self.checkmark_colour = Srgb::from(colour);
        self.header_colour = Srgb::from(colour);
        self.resize_grip_colour = Srgb::from(colour);
        self.tab_colour = Srgb::from(colour);

        self
    }

    ///
    /// Set the background colour to base the colour scheme on
    ///
    pub fn background(mut self, colour: [f32; 3]) -> Self {
        let colour: Hsv = Srgb::new(colour[0], colour[1], colour[2]).into_hsv();

        self.background_colour = Srgb::from(colour);
        self.scrollbar_colour = Srgb::from(Self::value_shift_mul(colour, 4.0, self.light_theme));
        self.frame_colour = Srgb::from(Self::value_shift_mul(colour, 0.5, self.light_theme));
        self.title_unfocused = Srgb::from(Self::value_shift_mul(colour, 0.2, self.light_theme));
        self
    }

    ///
    /// Apply the built colour scheme to the target style
    ///
    pub fn build(self, style: &mut render::imgui::Style) {
        let bg_base: Hsv = self.background_colour.into_hsv();
        let separator_base: Hsv = self.separator_colour.into_hsv();
        let title_base: Hsv = self.title_colour.into_hsv();
        let title_unfocused: Hsv = self.title_unfocused.into_hsv();
        let slider_base: Hsv = self.slider_colour.into_hsv();
        let scrollbar_base: Hsv = self.scrollbar_colour.into_hsv();
        let frame_base: Hsv = self.frame_colour.into_hsv();
        let border_base: Hsv = self.border_colour.into_hsv();
        let button_base: Hsv = self.button_colour.into_hsv();
        let checkmark_base: Hsv = self.checkmark_colour.into_hsv();
        let header_base: Hsv = self.header_colour.into_hsv();
        let resize_grip_base: Hsv = self.resize_grip_colour.into_hsv();
        let tab_base: Hsv = self.tab_colour.into_hsv();

        let text_base: Hsv = if self.light_theme {
            Hsv::new(0.0, 0.0, 0.0)
        } else {
            Hsv::new(0.0, 0.0, 1.0)
        };

        // Text
        Self::apply_colour(text_base, &mut style[StyleColor::Text], 1.0);

        // Border
        Self::apply_colour(border_base, &mut style[StyleColor::Border], 1.0);

        self.separator_colours(separator_base, style);
        self.window_background_colours(bg_base, style);
        self.scrollbar_colours(scrollbar_base, style);
        self.title_colours(title_base, style);
        self.title_unfocused_colours(title_unfocused, style);
        self.checkmark_colours(checkmark_base, style);
        self.slider_colours(slider_base, style);
        self.button_colours(button_base, style);
        self.frame_colours(frame_base, style);
        self.header_colours(Self::value_shift_mul(header_base, 0.6, false), style);
        self.resize_grip_colours(Self::value_shift_mul(resize_grip_base, 0.6, false), style);
        self.tab_colours(Self::value_shift_mul(tab_base, 0.6, false), style);
    }

    ///
    /// Apply the given colour to the separator colour category
    ///
    fn separator_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(colour, &mut style[StyleColor::SeparatorHovered], 1.0);

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.5, self.light_theme),
            &mut style[StyleColor::SeparatorActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the window background colour category
    ///
    fn window_background_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 50.0, self.light_theme),
            &mut style[StyleColor::Separator],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, self.light_theme),
            &mut style[StyleColor::WindowBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, self.light_theme),
            &mut style[StyleColor::ChildBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.7, self.light_theme),
            &mut style[StyleColor::PopupBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.5, self.light_theme),
            &mut style[StyleColor::MenuBarBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.5, self.light_theme),
            &mut style[StyleColor::ScrollbarBg],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the title colour category
    ///
    fn title_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(colour, &mut style[StyleColor::TitleBgActive], 1.0);
    }

    ///
    /// Apply the given colour to the title unfocused colour category
    ///
    fn title_unfocused_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::TitleBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.75, false),
            &mut style[StyleColor::TitleBgCollapsed],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the scrollbar colour category
    ///
    fn scrollbar_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.5, self.light_theme),
            &mut style[StyleColor::ScrollbarGrab],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, self.light_theme),
            &mut style[StyleColor::ScrollbarGrabHovered],
            1.0,
        );

        // Button Active
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.75, self.light_theme),
            &mut style[StyleColor::ScrollbarGrabActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the checkmark colour category
    ///
    fn checkmark_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(colour, &mut style[StyleColor::CheckMark], 1.0);
    }

    ///
    /// Apply the given colour to the slider colour category
    ///
    fn slider_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::SliderGrab],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::SliderGrabActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the button colour category
    ///
    fn button_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::Button],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::ButtonHovered],
            1.0,
        );

        // Button Active
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.7, false),
            &mut style[StyleColor::ButtonActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the frame colour category
    ///
    fn frame_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::FrameBg],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::FrameBgHovered],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.05, false),
            &mut style[StyleColor::FrameBgActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the header colour category
    ///
    fn header_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::Header],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::HeaderHovered],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.05, false),
            &mut style[StyleColor::HeaderActive],
            1.0,
        );
    }

    ///
    /// Apply the given colour to the resize grip colour category
    ///
    fn resize_grip_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::ResizeGrip],
            0.5,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::ResizeGripHovered],
            0.5,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.05, false),
            &mut style[StyleColor::ResizeGripActive],
            0.5,
        );
    }

    ///
    /// Apply the given colour to the tab colour category
    ///
    fn tab_colours(&self, colour: Hsv, style: &mut render::imgui::Style) {
        Self::apply_colour(
            Self::value_shift_mul(colour, 0.8, false),
            &mut style[StyleColor::Tab],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.0, false),
            &mut style[StyleColor::TabHovered],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 1.15, false),
            &mut style[StyleColor::TabActive],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.65, false),
            &mut style[StyleColor::TabUnfocused],
            1.0,
        );

        Self::apply_colour(
            Self::value_shift_mul(colour, 0.5, false),
            &mut style[StyleColor::TabUnfocusedActive],
            1.0,
        );
    }

    ///
    /// Increases value when dark theme, decreases value when light theme
    ///
    #[inline]
    fn value_shift_mul(value: Hsv, factor: f32, reciprocal: bool) -> Hsv {
        let val = if reciprocal {
            let factor = 1.0 / factor;
            value.value * factor
        } else {
            value.value * factor
        };
        Hsv::new(value.hue, value.saturation, val.min(1.0).max(0.0))
    }

    #[inline]
    fn apply_colour(apply: Hsv, to: &mut [f32; 4], alpha: f32) {
        let apply = Srgb::from(apply).into_components();
        to[0] = apply.0;
        to[1] = apply.1;
        to[2] = apply.2;
        to[3] = alpha;
    }
}

///
///
///
pub struct Imgui {
    context: render::imgui::Context,
    cursors: Vec<Cursor>,
}

impl Imgui {
    pub fn new() -> Self {
        aleph_log::trace!("Initializing ImGui");
        let mut context = render::imgui::Context::create();
        context.set_ini_filename(None);
        context.set_clipboard_backend(ImguiClipboard::new());
        context.style_mut().child_rounding = 0.0;
        context.style_mut().frame_rounding = 0.0;
        context.style_mut().grab_rounding = 0.0;
        context.style_mut().popup_rounding = 0.0;
        context.style_mut().scrollbar_rounding = 0.0;
        context.style_mut().tab_rounding = 0.0;
        context.style_mut().window_rounding = 0.0;

        ImguiStyleBuilder::dark().build(context.style_mut());

        let io = context.io_mut();
        io.backend_flags |= render::imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= render::imgui::BackendFlags::HAS_SET_MOUSE_POS;
        io.backend_flags |= render::imgui::BackendFlags::RENDERER_HAS_VTX_OFFSET;

        io.key_map[render::imgui::Key::Tab as usize] = Scancode::Tab as u32;
        io.key_map[render::imgui::Key::LeftArrow as usize] = Scancode::Left as u32;
        io.key_map[render::imgui::Key::RightArrow as usize] = Scancode::Right as u32;
        io.key_map[render::imgui::Key::UpArrow as usize] = Scancode::Up as u32;
        io.key_map[render::imgui::Key::DownArrow as usize] = Scancode::Down as u32;
        io.key_map[render::imgui::Key::PageUp as usize] = Scancode::PageUp as u32;
        io.key_map[render::imgui::Key::PageDown as usize] = Scancode::PageDown as u32;
        io.key_map[render::imgui::Key::Home as usize] = Scancode::Home as u32;
        io.key_map[render::imgui::Key::End as usize] = Scancode::End as u32;
        io.key_map[render::imgui::Key::Insert as usize] = Scancode::Insert as u32;
        io.key_map[render::imgui::Key::Delete as usize] = Scancode::Delete as u32;
        io.key_map[render::imgui::Key::Backspace as usize] = Scancode::Backspace as u32;
        io.key_map[render::imgui::Key::Space as usize] = Scancode::Space as u32;
        io.key_map[render::imgui::Key::Enter as usize] = Scancode::Return as u32;
        io.key_map[render::imgui::Key::Escape as usize] = Scancode::Escape as u32;
        io.key_map[render::imgui::Key::KeyPadEnter as usize] = Scancode::KpEnter as u32;
        io.key_map[render::imgui::Key::A as usize] = Scancode::A as u32;
        io.key_map[render::imgui::Key::C as usize] = Scancode::C as u32;
        io.key_map[render::imgui::Key::V as usize] = Scancode::V as u32;
        io.key_map[render::imgui::Key::X as usize] = Scancode::X as u32;
        io.key_map[render::imgui::Key::Y as usize] = Scancode::Y as u32;
        io.key_map[render::imgui::Key::Z as usize] = Scancode::Z as u32;

        let cursors = vec![
            Cursor::Arrow,
            Cursor::IBeam,
            Cursor::SizeAll,
            Cursor::SizeNS,
            Cursor::SizeWE,
            Cursor::SizeNESW,
            Cursor::SizeNWSE,
            Cursor::Hand,
            Cursor::No,
        ];

        aleph_log::trace!("");
        Self { context, cursors }
    }

    ///
    /// There's some stuff we need to run before we handle mouse events and requests and some other
    /// stuff we need to run after mouse events so we split them into different functions
    ///
    pub fn update_mouse_pos_early(&mut self) {
        let io = self.context.io_mut();

        if io.want_set_mouse_pos {
            Mouse::set_pos(io.mouse_pos[0] as i32, io.mouse_pos[1] as i32);
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
    pub fn frame(&mut self) -> render::imgui::Ui {
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

        if (ui.io().config_flags & render::imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
            == render::imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE
        {
            return ui;
        }

        let cursor = if let Some(cursor) = ui.mouse_cursor() {
            cursor
        } else {
            return ui;
        };

        if ui.io().mouse_draw_cursor || cursor as i32 == render::imgui::sys::ImGuiMouseCursor_None {
            Mouse::hide_cursor();
        } else {
            Mouse::show_cursor();
            Mouse::set_cursor(self.cursors[cursor as usize]);
        }

        ui
    }

    pub fn context_mut(&mut self) -> &mut render::imgui::Context {
        &mut self.context
    }
}

struct ImguiClipboard {}

impl ImguiClipboard {
    fn new() -> Box<Self> {
        Box::new(Self {})
    }
}

impl render::imgui::ClipboardBackend for ImguiClipboard {
    fn get(&mut self) -> Option<ImString> {
        // Gets the string data
        let cstring = Clipboard::get_null_terminated()?;

        // Ensure the string is valid UTF-8
        cstring.to_str().ok()?;

        // Get the bytes out of the string
        let bytes = cstring.into_bytes_with_nul();

        // Convert to an ImString, this is safe as we know it's null terminated because a `CString`
        // is always null terminated. We know the string is valid UTF8 as we checked for this
        // explicitly ourselves
        unsafe { Some(ImString::from_utf8_with_nul_unchecked(bytes)) }
    }

    fn set(&mut self, value: &ImStr) {
        Clipboard::set_null_terminated(value.as_ref());
    }
}
