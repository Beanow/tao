// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

// System tray is supported and availabled only if `tray` feature is enabled.
// Platform: Windows, Linux and macOS.
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[cfg(any(feature = "tray", all(target_os = "linux", feature = "ayatana")))]
fn main() {
  #[cfg(target_os = "macos")]
  use tao::platform::macos::{CustomMenuItemExtMacOS, NativeImage, SystemTrayBuilderExtMacOS};
  use tao::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    menu::{ContextMenu as Menu, MenuItemAttributes, MenuType},
    system_tray::SystemTrayBuilder,
  };

  env_logger::init();
  let event_loop = EventLoop::new();

  let mut tray_menu = Menu::new();
  let quit = tray_menu.add_item(MenuItemAttributes::new("Quit"));

  // You'll have to choose an icon size at your own discretion. On Linux, the icon should be
  // provided in whatever size it was naturally drawn; that is, don’t scale the image before passing
  // it to Tao. But on Windows, you will have to account for screen scaling. Here we use 32px,
  // since it seems to work well enough in most cases. Be careful about going too high, or
  // you'll be bitten by the low-quality downscaling built into the WM.
  let path = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/icon.png");

  let icon = load_icon(std::path::Path::new(path));

  let system_tray = SystemTrayBuilder::new(icon, Some(tray_menu))
    .build(&event_loop)
    .unwrap();

  event_loop.run(move |event, _event_loop, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::MenuEvent {
        menu_id,
        // specify only context menu's
        origin: MenuType::ContextMenu,
        ..
      } => {
        if menu_id == quit.clone().id() {
          // drop the system tray before exiting to remove the icon from system tray on Windows
          drop(&system_tray);
          *control_flow = ControlFlow::Exit;
        }
      }
      _ => (),
    }
  });
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[cfg(any(feature = "tray", all(target_os = "linux", feature = "ayatana")))]
fn load_icon(path: &std::path::Path) -> tao::system_tray::Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open(path)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  tao::system_tray::Icon::from_rgba(icon_rgba, icon_width, icon_height)
    .expect("Failed to open icon")
}

// System tray isn't supported on other's platforms.
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn main() {
  println!("This platform doesn't support system_tray.");
}

// Tray feature flag disabled but can be available.
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
#[cfg(not(feature = "tray"))]
fn main() {
  println!("This platform doesn't have the `tray` feature enabled.");
}
