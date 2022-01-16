use anyhow::bail;
use base::Gamemode;
use common::{window::BackingWindow, Window};
use ecs::{EntityRef, SysResult};
use protocol::packets::client::{ClickWindow, CreativeInventoryAction};

use crate::{ClientId, Server};

pub fn handle_creative_inventory_action(
    player: EntityRef,
    packet: CreativeInventoryAction,
    server: &mut Server,
) -> SysResult {
    if *player.get::<Gamemode>()? != Gamemode::Creative {
        bail!("cannot use Creative Inventory Action outside of creative mode");
    }

    if packet.slot != -1 {
        let window = player.get::<Window>()?;
        if !matches!(window.inner(), BackingWindow::Player { .. }) {
            bail!("cannot use Creative Inventory Action in external inventories");
        }

        window
            .inner()
            .set_item(packet.slot as usize, packet.clicked_item)?;

        // Sends the client updates about window changes.
        // Is required to make delete inventory button reflect in-game.
        let client_id = *player.get::<ClientId>()?;
        let client = server.clients.get(client_id).unwrap();
        client.send_window_items(&window);
    }

    Ok(())
}

pub fn handle_click_window(
    server: &mut Server,
    player: EntityRef,
    packet: ClickWindow,
) -> SysResult {
    let mut window = player.get_mut::<Window>().unwrap();
    let result = _handle_click_window(&packet, &mut window);

    let client = server
        .clients
        .get(*player.get::<ClientId>().unwrap())
        .unwrap();

    if packet.slot >= 0 {
        let item = window.item(packet.slot as usize)?.clone();
        let old_cursor_item = window.cursor_item();
        client.send_inventory_slot(packet.slot, old_cursor_item);
        window.set_cursor_item(item);
    }
    client.send_cursor_slot(window.cursor_item());

    client.send_window_items(&*window);

    result
}

fn _handle_click_window(packet: &ClickWindow, window: &mut Window) -> SysResult {
    match packet.mode {
        0 => match packet.button {
            0 => window.left_click(packet.slot as usize)?,
            1 => window.right_click(packet.slot as usize)?,
            _ => bail!("unrecgonized click"),
        },
        1 => window.shift_click(packet.slot as usize)?,
        5 => match packet.button {
            0 => window.begin_left_mouse_paint(),
            4 => window.begin_right_mouse_paint(),
            1 | 5 => window.add_paint_slot(packet.slot as usize)?,
            2 | 6 => window.end_paint()?,
            _ => bail!("unrecognized paint operation"),
        },
        _ => bail!("unsupported window click mode"),
    };
    Ok(())
}
