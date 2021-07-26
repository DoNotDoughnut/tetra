//! Functions and types relating to the game window, and the environment it is running in.

use crate::{context::TetraContext, Result};

/// Quits the game, if it is currently running.
///
/// Note that quitting the game does not take effect until the end of the current
/// cycle of the game loop. This will probably change later.
pub fn quit(ctx: &mut TetraContext) {
    ctx.running = false;
}

/// Gets the current title of the window.
pub fn get_title(ctx: &TetraContext) -> &str {
    ctx.window.get_window_title()
}

/// Sets the title of the window.
pub fn set_title<S>(ctx: &mut TetraContext, title: S)
where
    S: AsRef<str>,
{
    ctx.window.set_window_title(title)
}

/// Gets the width of the window.
///
/// This function will return a consistent value regardless of whether
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled. To find
/// the physical width of the window, call [`get_physical_width`].
pub fn get_width(ctx: &TetraContext) -> i32 {
    ctx.window.get_window_size().0
}

/// Sets the width of the window.
///
/// # Errors
///
/// * [`TetraError::FailedToChangeDisplayMode`](crate::TetraError::FailedToChangeDisplayMode)
/// will be returned if the game was unable to change the window size.
pub fn set_width(ctx: &mut TetraContext, width: i32) -> Result {
    set_size(ctx, width, ctx.window.get_window_size().1)
}

/// Gets the height of the window.
///
/// This function will return a consistent value regardless of whether
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled. To find
/// the physical height of the window, call [`get_physical_height`].
pub fn get_height(ctx: &TetraContext) -> i32 {
    ctx.window.get_window_size().1
}

/// Sets the height of the window.
///
/// # Errors
///
/// * [`TetraError::FailedToChangeDisplayMode`](crate::TetraError::FailedToChangeDisplayMode)
/// will be returned if the game was unable to change the window size.
pub fn set_height(ctx: &mut TetraContext, height: i32) -> Result {
    set_size(ctx, ctx.window.get_window_size().0, height)
}

/// Gets the size of the window.
///
/// This function will return a consistent value regardless of whether
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled. To find
/// the physical size of the window, call [`get_physical_size`].
pub fn get_size(ctx: &TetraContext) -> (i32, i32) {
    ctx.window.get_window_size()
}

/// Sets the size of the window.
///
/// # Errors
///
/// * [`TetraError::FailedToChangeDisplayMode`](crate::TetraError::FailedToChangeDisplayMode)
/// will be returned if the game was unable to change the window size.
pub fn set_size(ctx: &mut TetraContext, width: i32, height: i32) -> Result {
    ctx.window.set_window_size(width, height)
}

/// Returns the width of the window in physical pixels.
///
/// The output of this function may differ from the output of [`get_width`] if
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled.
pub fn get_physical_width(ctx: &TetraContext) -> i32 {
    ctx.window.get_physical_size().0
}

/// Returns the height of the window in physical pixels.
///
/// The output of this function may differ from the output of [`get_height`] if
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled.
pub fn get_physical_height(ctx: &TetraContext) -> i32 {
    ctx.window.get_physical_size().1
}

/// Returns the size of the window in physical pixels.
///
/// The output of this function may differ from the output of [`get_size`] if
/// [high DPI support](crate::ContextBuilder::high_dpi) is enabled.
pub fn get_physical_size(ctx: &TetraContext) -> (i32, i32) {
    ctx.window.get_physical_size()
}

/// Returns the ratio of the logical resolution to the physical resolution of the current
/// display on which the window is being displayed.
///
/// This will usually be `1.0`, but if [high DPI support](crate::ContextBuilder::high_dpi)
/// is enabled and the monitor is high DPI, it may be higher. For example, on a Mac with
/// a retina display, this can return `2.0`.
pub fn get_dpi_scale(ctx: &TetraContext) -> f32 {
    ctx.window.get_dpi_scale()
}

/// Returns whether the window is currently visible, or whether it has been hidden.
///
/// Note that a minimized window is still considered 'visible', as the user is able
/// to restore it if they want to.
pub fn is_visible(ctx: &mut TetraContext) -> bool {
    ctx.window.is_visible()
}

/// Sets whether or not the window should be visible to the user.
pub fn set_visible(ctx: &mut TetraContext, visible: bool) {
    ctx.window.set_visible(visible);
}

/// Sets whether the window should be vsynced.
///
/// # Errors
///
/// * [`TetraError::FailedToChangeDisplayMode`](crate::TetraError::FailedToChangeDisplayMode)
/// will be returned if the game was unable to change vsync mode.
pub fn set_vsync(ctx: &mut TetraContext, vsync: bool) -> Result {
    ctx.window.set_vsync(vsync)
}

/// Returns whether or not vsync is enabled.
pub fn is_vsync_enabled(ctx: &TetraContext) -> bool {
    ctx.window.is_vsync_enabled()
}

/// Sets whether the window should be in fullscreen mode.
///
/// # Errors
///
/// * [`TetraError::FailedToChangeDisplayMode`](crate::TetraError::FailedToChangeDisplayMode)
/// will be returned if the game was unable to enter or exit fullscreen.
pub fn set_fullscreen(ctx: &mut TetraContext, fullscreen: bool) -> Result {
    ctx.window.set_fullscreen(fullscreen)
}

/// Returns whether or not the window is currently in fullscreen mode.
pub fn is_fullscreen(ctx: &TetraContext) -> bool {
    ctx.window.is_fullscreen()
}

/// Sets whether or not the mouse cursor should be visible.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned if
/// the cursor state was inaccessible.
pub fn set_mouse_visible(ctx: &mut TetraContext, visible: bool) -> Result {
    ctx.window.set_mouse_visible(visible)
}

/// Returns whether or not the mouse cursor is currently visible.
pub fn is_mouse_visible(ctx: &TetraContext) -> bool {
    ctx.window.is_mouse_visible()
}

/// Sets whether or not the mouse is grabbed by the window.
///
/// When this is active, the cursor will not be able to leave the window while it
/// is focused.
pub fn set_mouse_grabbed(ctx: &mut TetraContext, mouse_grabbed: bool) {
    ctx.window.set_mouse_grabbed(mouse_grabbed);
}

/// Returns whether or not the mouse is currently grabbed by the window.
///
/// When this is active, the cursor will not be able to leave the window while it
/// is focused.
pub fn is_mouse_grabbed(ctx: &TetraContext) -> bool {
    ctx.window.is_mouse_grabbed()
}

/// Sets whether or not relative mouse mode is enabled.
///
/// While the mouse is in relative mode, the cursor is hidden and can move beyond the
/// bounds of the window. The `delta` field of [`Event::MouseMoved`](crate::Event::MouseMoved)
/// can then be used to track the cursor's changes in position. This is useful when
/// implementing control schemes that require the mouse to be able to move infinitely
/// in any direction (for example, FPS-style movement).
///
/// While this mode is enabled, the absolute position of the mouse may not be updated -
/// as such, you should not rely on it.
pub fn set_relative_mouse_mode(ctx: &mut TetraContext, relative_mouse_mode: bool) {
    ctx.window.set_relative_mouse_mode(relative_mouse_mode);
}

/// Returns whether or not relative mouse mode is currently enabled.
///
/// While the mouse is in relative mode, the cursor is hidden and can move beyond the
/// bounds of the window. The `delta` field of [`Event::MouseMoved`](crate::Event::MouseMoved)
/// can then be used to track the cursor's changes in position. This is useful when
/// implementing control schemes that require the mouse to be able to move infinitely
/// in any direction (for example, FPS-style movement).
///
/// While this mode is enabled, the absolute position of the mouse may not be updated -
/// as such, you should not rely on it.
pub fn is_relative_mouse_mode(ctx: &TetraContext) -> bool {
    ctx.window.is_relative_mouse_mode()
}

/// Gets the number of monitors connected to the device.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_monitor_count(ctx: &TetraContext) -> Result<i32> {
    ctx.window.get_monitor_count()
}

/// Gets the name of a monitor connected to the device.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_monitor_name(ctx: &TetraContext, monitor_index: i32) -> Result<String> {
    ctx.window.get_monitor_name(monitor_index)
}

/// Gets the width of a monitor connected to the device.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_monitor_width(ctx: &TetraContext, monitor_index: i32) -> Result<i32> {
    get_monitor_size(ctx, monitor_index).map(|(w, _)| w)
}

/// Gets the height of a monitor connected to the device.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_monitor_height(ctx: &TetraContext, monitor_index: i32) -> Result<i32> {
    get_monitor_size(ctx, monitor_index).map(|(_, h)| h)
}

/// Gets the size of a monitor connected to the device.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_monitor_size(ctx: &TetraContext, monitor_index: i32) -> Result<(i32, i32)> {
    ctx.window.get_monitor_size(monitor_index)
}

/// Gets the index of the monitor that the window is currently on.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_current_monitor(ctx: &TetraContext) -> Result<i32> {
    ctx.window.get_current_monitor()
}

/// Gets the name of the monitor that the window is currently on.
///
/// # Errors
///
/// * [[`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_current_monitor_name(ctx: &TetraContext) -> Result<String> {
    let monitor_index = ctx.window.get_current_monitor()?;
    ctx.window.get_monitor_name(monitor_index)
}

/// Gets the width of the monitor that the window is currently on.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_current_monitor_width(ctx: &TetraContext) -> Result<i32> {
    get_current_monitor_size(ctx).map(|(w, _)| w)
}

/// Gets the height of the monitor that the window is currently on.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_current_monitor_height(ctx: &TetraContext) -> Result<i32> {
    get_current_monitor_size(ctx).map(|(_, h)| h)
}

/// Gets the size of the monitor that the window is currently on.
///
/// # Errors
///
/// * [`TetraError::PlatformError`](crate::TetraError::PlatformError) will be returned
/// if the monitor state was inaccessible.
pub fn get_current_monitor_size(ctx: &TetraContext) -> Result<(i32, i32)> {
    let monitor_index = ctx.window.get_current_monitor()?;
    ctx.window.get_monitor_size(monitor_index)
}

/// Sets whether or not the user's screen saver can be displayed while the game is running.
pub fn set_screen_saver_enabled(ctx: &TetraContext, screen_saver_enabled: bool) {
    ctx.window.set_screen_saver_enabled(screen_saver_enabled);
}

/// Returns whether or not the user's screen saver can be displayed while the game is running.
pub fn is_screen_saver_enabled(ctx: &TetraContext) -> bool {
    ctx.window.is_screen_saver_enabled()
}

/// Sets whether or not key repeat should be enabled.
///
/// Normally, a [`KeyPressed`](crate::Event::KeyPressed) event will only be fired once, when
/// the key is initially pressed. Enabling key repeat causes `KeyPressed` events to be fired
/// continuously while the key is held down.
pub fn set_key_repeat_enabled(ctx: &mut TetraContext, key_repeat_enabled: bool) {
    ctx.window.set_key_repeat_enabled(key_repeat_enabled);
}

/// Returns whether or not key repeat is enabled.
///
/// Normally, a [`KeyPressed`](crate::Event::KeyPressed) event will only be fired once, when
/// the key is initially pressed. Enabling key repeat causes `KeyPressed` events to be fired
/// continuously while the key is held down.
pub fn is_key_repeat_enabled(ctx: &TetraContext) -> bool {
    ctx.window.is_key_repeat_enabled()
}
