//! This is the interface to the protobuf messages.

use serde::Serialize;

use crate::cursor::Cursor;
use crate::protos;

#[derive(Debug, Serialize)]
pub enum Message {
    Nop(protos::CnetMsgNop),
    Disconnect(protos::CnetMsgDisconnect),
    File(protos::CnetMsgFile),
    Tick(protos::CnetMsgTick),
    StringCmd(protos::CnetMsgStringCmd),
    SetConVar(protos::CnetMsgSetConVar),
    SignonState(protos::CnetMsgSignonState),
    SplitScreenUser(protos::CnetMsgSplitScreenUser),
    PlayerAvatarData(protos::CnetMsgPlayerAvatarData),
    ServerInfo(protos::CsvcMsgServerInfo),
    SendTable(protos::CsvcMsgSendTable),
    ClassInfo(protos::CsvcMsgClassInfo),
    SetPause(protos::CsvcMsgSetPause),
    CreateStringTable(protos::CsvcMsgCreateStringTable),
    UpdateStringTable(protos::CsvcMsgUpdateStringTable),
    VoiceInit(protos::CsvcMsgVoiceInit),
    VoiceData(protos::CsvcMsgVoiceData),
    Print(protos::CsvcMsgPrint),
    Sounds(protos::CsvcMsgSounds),
    SetView(protos::CsvcMsgSetView),
    FixAngle(protos::CsvcMsgFixAngle),
    CrosshairAngle(protos::CsvcMsgCrosshairAngle),
    BSPDecal(protos::CsvcMsgBspDecal),
    UserMessage(protos::CsvcMsgUserMessage),
    GameEvent(protos::CsvcMsgGameEvent),
    PacketEntities(protos::CsvcMsgPacketEntities),
    TempEntities(protos::CsvcMsgTempEntities),
    Prefetch(protos::CsvcMsgPrefetch),
    Menu(protos::CsvcMsgMenu),
    GameEventList(protos::CsvcMsgGameEventList),
    GetCvarValue(protos::CsvcMsgGetCvarValue),
    SplitScreen(protos::CsvcMsgSplitScreen),
    EntityMessage(protos::CsvcMsgEntityMsg),
    PaintmapData(protos::CsvcMsgPaintmapData),
    CmdKeyValues(protos::CsvcMsgCmdKeyValues),
    EncryptedData(protos::CsvcMsgEncryptedData),
    HltvReplay(protos::CsvcMsgHltvReplay),
    BroadcastCommand(protos::CsvcMsgBroadcastCommand),
}

fn make<T: Default + prost::Message>(data: &Cursor, length: u32) -> anyhow::Result<T> {
    Ok(T::decode(data.read_bytes(length as usize)?.as_ref())?)
}

impl Message {
    pub fn parse(data: &Cursor, message_type: i32, length: u32) -> anyhow::Result<Message> {
        //protos::NetMessages::from_i32(which).map(|nm|
        // MessageType::Net(nm)).or_else(||
        // protos::SvcMessages::from_i32(which).unwrap_or_else(|| anyhow::anyhow!("Bad
        // ordinal for Message Type: `{which}`.")))
        let msg = if let Some(nm) = protos::NetMessages::from_i32(message_type) {
            match nm {
                protos::NetMessages::NetNop => Message::Nop(make(data, length)?),
                protos::NetMessages::NetDisconnect => Message::Disconnect(make(data, length)?),
                protos::NetMessages::NetFile => Message::File(make(data, length)?),
                protos::NetMessages::NetSplitScreenUser => {
                    Message::SplitScreenUser(make(data, length)?)
                }
                protos::NetMessages::NetTick => Message::Tick(make(data, length)?),
                protos::NetMessages::NetStringCmd => Message::StringCmd(make(data, length)?),
                protos::NetMessages::NetSetConVar => Message::SetConVar(make(data, length)?),
                protos::NetMessages::NetSignonState => Message::SignonState(make(data, length)?),
                protos::NetMessages::NetPlayerAvatarData => {
                    Message::PlayerAvatarData(make(data, length)?)
                }
            }
        } else if let Some(sm) = protos::SvcMessages::from_i32(message_type) {
            match sm {
                protos::SvcMessages::SvcServerInfo => Message::ServerInfo(make(data, length)?),
                protos::SvcMessages::SvcSendTable => Message::SendTable(make(data, length)?),
                protos::SvcMessages::SvcClassInfo => Message::ClassInfo(make(data, length)?),
                protos::SvcMessages::SvcSetPause => Message::SetPause(make(data, length)?),
                protos::SvcMessages::SvcCreateStringTable => {
                    Message::CreateStringTable(make(data, length)?)
                }
                protos::SvcMessages::SvcUpdateStringTable => {
                    Message::UpdateStringTable(make(data, length)?)
                }
                protos::SvcMessages::SvcVoiceInit => Message::VoiceInit(make(data, length)?),
                protos::SvcMessages::SvcVoiceData => Message::VoiceData(make(data, length)?),
                protos::SvcMessages::SvcPrint => Message::Print(make(data, length)?),
                protos::SvcMessages::SvcSounds => Message::Sounds(make(data, length)?),
                protos::SvcMessages::SvcSetView => Message::SetView(make(data, length)?),
                protos::SvcMessages::SvcFixAngle => Message::FixAngle(make(data, length)?),
                protos::SvcMessages::SvcCrosshairAngle => {
                    Message::CrosshairAngle(make(data, length)?)
                }
                protos::SvcMessages::SvcBspDecal => Message::BSPDecal(make(data, length)?),
                protos::SvcMessages::SvcSplitScreen => Message::SplitScreen(make(data, length)?),
                protos::SvcMessages::SvcUserMessage => Message::UserMessage(make(data, length)?),
                protos::SvcMessages::SvcEntityMessage => {
                    Message::EntityMessage(make(data, length)?)
                }
                protos::SvcMessages::SvcGameEvent => Message::GameEvent(make(data, length)?),
                protos::SvcMessages::SvcPacketEntities => {
                    Message::PacketEntities(make(data, length)?)
                }
                protos::SvcMessages::SvcTempEntities => Message::TempEntities(make(data, length)?),
                protos::SvcMessages::SvcPrefetch => Message::Prefetch(make(data, length)?),
                protos::SvcMessages::SvcMenu => Message::Menu(make(data, length)?),
                protos::SvcMessages::SvcGameEventList => {
                    Message::GameEventList(make(data, length)?)
                }
                protos::SvcMessages::SvcGetCvarValue => Message::GetCvarValue(make(data, length)?),
                protos::SvcMessages::SvcPaintmapData => Message::PaintmapData(make(data, length)?),
                protos::SvcMessages::SvcCmdKeyValues => Message::CmdKeyValues(make(data, length)?),
                protos::SvcMessages::SvcEncryptedData => {
                    Message::EncryptedData(make(data, length)?)
                }
                protos::SvcMessages::SvcHltvReplay => Message::HltvReplay(make(data, length)?),
                protos::SvcMessages::SvcBroadcastCommand => {
                    Message::BroadcastCommand(make(data, length)?)
                }
            }
        } else {
            anyhow::bail!("Bad ordinal for Message Type: `{message_type}`.");
        };
        Ok(msg)
    }
}
