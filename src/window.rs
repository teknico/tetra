//! Functions and types relating to the game window, and the environment it is running in.

use crate::{Context, Result};

/// Quits the game, if it is currently running.
///
/// Note that quitting the game does not take effect until the end of the current
/// cycle of the game loop. This will probably change later.
pub fn quit(ctx: &mut Context) {
    ctx.running = false;
}

/// Gets the current title of the window.
pub fn get_title(ctx: &Context) -> &str {
    ctx.window.get_window_title()
}

/// Sets the title of the window.
pub fn set_title<S>(ctx: &mut Context, title: S)
where
    S: AsRef<str>,
{
    ctx.window.set_window_title(title)
}

/// Gets the width of the window.
pub fn get_width(ctx: &Context) -> i32 {
    ctx.window.get_window_width()
}

/// Sets the width of the window.
///
/// # Errors
///
/// * `TetraError::FailedToChangeDisplayMode` will be returned if the game was unable to
/// change the window size.
pub fn set_width(ctx: &mut Context, width: i32) -> Result {
    set_size(ctx, width, ctx.window.get_window_height())
}

/// Gets the height of the window.
pub fn get_height(ctx: &Context) -> i32 {
    ctx.window.get_window_height()
}

/// Sets the height of the window.
///
/// # Errors
///
/// * `TetraError::FailedToChangeDisplayMode` will be returned if the game was unable to
/// change the window size.
pub fn set_height(ctx: &mut Context, height: i32) -> Result {
    set_size(ctx, ctx.window.get_window_width(), height)
}

/// Gets the size of the window.
pub fn get_size(ctx: &Context) -> (i32, i32) {
    ctx.window.get_window_size()
}

/// Sets the size of the window.
///
/// # Errors
///
/// * `TetraError::FailedToChangeDisplayMode` will be returned if the game was unable to
/// change the window size.
pub fn set_size(ctx: &mut Context, width: i32, height: i32) -> Result {
    ctx.window.set_window_size(width, height)
}

/// Sets whether the window should be vsynced.
///
/// # Errors
///
/// * `TetraError::FailedToChangeDisplayMode` will be returned if the game was unable to
/// change vsync mode.
pub fn set_vsync(ctx: &mut Context, vsync: bool) -> Result {
    ctx.window.set_vsync(vsync)
}

/// Returns whethere or not vsync is enabled.
pub fn is_vsync_enabled(ctx: &Context) -> bool {
    ctx.window.is_vsync_enabled()
}

/// Sets whether the window should be in fullscreen mode.
///
/// # Errors
///
/// * `TetraError::FailedToChangeDisplayMode` will be returned if the game was unable to
/// enter or exit fullscreen.
pub fn set_fullscreen(ctx: &mut Context, fullscreen: bool) -> Result {
    ctx.window.set_fullscreen(fullscreen)
}

/// Returns whether or not the window is currently in fullscreen mode.
pub fn is_fullscreen(ctx: &Context) -> bool {
    ctx.window.is_fullscreen()
}

/// Sets whether or not the mouse cursor should be visible.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the cursor state was inaccessible.
pub fn set_mouse_visible(ctx: &mut Context, visible: bool) -> Result {
    ctx.window.set_mouse_visible(visible)
}

/// Returns whether or not the mouse cursor is currently visible.
pub fn is_mouse_visible(ctx: &Context) -> bool {
    ctx.window.is_mouse_visible()
}

/// Get the number of monitors connected to the device.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_monitor_count(ctx: &Context) -> Result<i32> {
    ctx.window.get_monitor_count()
}

/// Get the name of a monitor connected to the device.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_monitor_name(ctx: &Context, monitor_index: i32) -> Result<String> {
    ctx.window.get_monitor_name(monitor_index)
}

/// Get the width of a monitor connected to the device.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_monitor_width(ctx: &Context, monitor_index: i32) -> Result<i32> {
    get_monitor_size(ctx, monitor_index).map(|(w, _)| w)
}

/// Get the height of a monitor connected to the device.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_monitor_height(ctx: &Context, monitor_index: i32) -> Result<i32> {
    get_monitor_size(ctx, monitor_index).map(|(_, h)| h)
}

/// Get the size of a monitor connected to the device.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_monitor_size(ctx: &Context, monitor_index: i32) -> Result<(i32, i32)> {
    ctx.window.get_monitor_size(monitor_index)
}

/// Get the index of the monitor that the window is currently on.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_current_monitor(ctx: &Context) -> Result<i32> {
    ctx.window.get_current_monitor()
}

/// Get the name of the monitor that the window is currently on.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_current_monitor_name(ctx: &Context) -> Result<String> {
    let monitor_index = ctx.window.get_current_monitor()?;
    ctx.window.get_monitor_name(monitor_index)
}

/// Get the width of the monitor that the window is currently on.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_current_monitor_width(ctx: &Context) -> Result<i32> {
    get_current_monitor_size(ctx).map(|(w, _)| w)
}

/// Get the height of the monitor that the window is currently on.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_current_monitor_height(ctx: &Context) -> Result<i32> {
    get_current_monitor_size(ctx).map(|(_, h)| h)
}

/// Get the size of the monitor that the window is currently on.
///
/// # Errors
///
/// * `TetraError::PlatformError` will be returned if the monitor state was inaccessible.
pub fn get_current_monitor_size(ctx: &Context) -> Result<(i32, i32)> {
    let monitor_index = ctx.window.get_current_monitor()?;
    ctx.window.get_monitor_size(monitor_index)
}
