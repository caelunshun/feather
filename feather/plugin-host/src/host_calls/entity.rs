use feather_base::Text;
use feather_common::chat::{ChatKind, ChatMessage};
use feather_ecs::Entity;
use feather_plugin_host_macros::host_function;

use crate::context::{PluginContext, PluginPtr};

#[host_function]
pub fn entity_exists(cx: &PluginContext, entity: u64) -> anyhow::Result<u32> {
    Ok(cx.game_mut().ecs.entity(Entity::from_bits(entity)).is_ok()).map(|b| b as u32)
}

#[host_function]
pub fn entity_send_message(
    cx: &PluginContext,
    entity: u64,
    message_ptr: PluginPtr<u8>,
    message_len: u32,
) -> anyhow::Result<()> {
    let message = cx.read_string(message_ptr, message_len)?;
    let entity = Entity::from_bits(entity);
    let _ = cx.game_mut().send_message(
        entity,
        ChatMessage::new(ChatKind::System, Text::of(message)),
    );
    Ok(())
}
