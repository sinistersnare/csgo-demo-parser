//! This is the interface to the protobuf messages.

use quick_protobuf::{BytesReader, MessageRead};

use crate::cursor::Cursor;
use crate::protos::netmessages::{self, SVC_Messages, NET_Messages};

#[derive(Debug, Clone)]
pub enum MessageType {
    Net(netmessages::NET_Messages),
    Svc(netmessages::SVC_Messages),
}

impl MessageType {
    pub fn from_ordinal(which: i32) -> anyhow::Result<MessageType> {
        match which {
            0..=7 | 100 => Ok(MessageType::Net(NET_Messages::from(which))),
            8..=31 | 33..=36 | 48 => Ok(MessageType::Svc(SVC_Messages::from(which)))
        }
    }
}

#[derive(Debug)]
pub enum Message<'a> {
    NOP(netmessages::CNETMsg_NOP),
    Disconnect(netmessages::CNETMsg_Disconnect<'a>),
    File(netmessages::CNETMsg_File<'a>),
    Tick(netmessages::CNETMsg_Tick),
    StringCmd(netmessages::CNETMsg_StringCmd<'a>),
    SetConVar(netmessages::CNETMsg_SetConVar<'a>),
    SignonState(netmessages::CNETMsg_SignonState<'a>),
    SplitScreenUser(netmessages::CNETMsg_SplitScreenUser),
    PlayerAvatarData(netmessages::CNETMsg_PlayerAvatarData<'a>),
    ServerInfo(netmessages::CSVCMsg_ServerInfo<'a>),
    SendTable(netmessages::CSVCMsg_SendTable<'a>),
    ClassInfo(netmessages::CSVCMsg_ClassInfo<'a>),
    SetPause(netmessages::CSVCMsg_SetPause),
    CreateStringTable(netmessages::CSVCMsg_CreateStringTable<'a>),
    UpdateStringTable(netmessages::CSVCMsg_UpdateStringTable<'a>),
    VoiceInit(netmessages::CSVCMsg_VoiceInit<'a>),
    VoiceData(netmessages::CSVCMsg_VoiceData<'a>),
    Print(netmessages::CSVCMsg_Print<'a>),
    Sounds(netmessages::CSVCMsg_Sounds),
    SetView(netmessages::CSVCMsg_SetView),
    FixAngle(netmessages::CSVCMsg_FixAngle),
    CrosshairAngle(netmessages::CSVCMsg_CrosshairAngle),
    BSPDecal(netmessages::CSVCMsg_BSPDecal),
    UserMessage(netmessages::CSVCMsg_UserMessage<'a>),
    GameEvent(netmessages::CSVCMsg_GameEvent<'a>),
    PacketEntities(netmessages::CSVCMsg_PacketEntities<'a>),
    TempEntities(netmessages::CSVCMsg_TempEntities<'a>),
    Prefetch(netmessages::CSVCMsg_Prefetch),
    Menu(netmessages::CSVCMsg_Menu<'a>),
    GameEventList(netmessages::CSVCMsg_GameEventList<'a>),
    GetCvarValue(netmessages::CSVCMsg_GetCvarValue<'a>),
    SplitScreen(netmessages::CSVCMsg_SplitScreen),
    EntityMessage(netmessages::CSVCMsg_EntityMsg<'a>),
    PaintmapData(netmessages::CSVCMsg_PaintmapData<'a>),
    CmdKeyValues(netmessages::CSVCMsg_CmdKeyValues<'a>),
    EncryptedData(netmessages::CSVCMsg_EncryptedData<'a>),
    HltvReplay(netmessages::CSVCMsg_HltvReplay),
    BroadcastCommand(netmessages::CSVCMsg_Broadcast_Command<'a>),
}

fn make<'a, T: MessageRead<'a>>(data: &'a Cursor, length: u32) -> anyhow::Result<T> {
    let buf = data.read_bytes(length as usize)?;
    let mut reader = BytesReader::from_bytes(buf);
    let built = T::from_reader(&mut reader, buf)?;
    Ok(built)
}

impl<'a> Message<'a> {
    pub fn new(
        data: &'a Cursor,
        message_type: MessageType,
        length: u32,
    ) -> anyhow::Result<Message<'a>> {
        let message = match message_type {
            MessageType::Net(NET_Messages::net_NOP) => Message::NOP(make(data, length)?),
            MessageType::Net(NET_Messages::net_Disconnect) => Message::Disconnect(make(data, length)?),
            MessageType::Net(NET_Messages::net_File) => Message::File(make(data, length)?),
            MessageType::Net(NET_Messages::net_Tick) => Message::Tick(make(data, length)?),
            MessageType::Net(NET_Messages::net_StringCmd) => Message::StringCmd(make(data, length)?),
            MessageType::Net(NET_Messages::net_SetConVar) => Message::SetConVar(make(data, length)?),
            MessageType::Net(NET_Messages::net_SignonState) => Message::SignonState(make(data, length)?),
            MessageType::Net(NET_Messages::net_SplitScreenUser) => Message::SplitScreenUser(make(data, length)?),
            MessageType::Net(NET_Messages::net_PlayerAvatarData) => Message::PlayerAvatarData(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_ServerInfo) => Message::ServerInfo(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_SendTable) => Message::SendTable(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_ClassInfo) => Message::ClassInfo(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_SetPause) => Message::SetPause(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_CreateStringTable) => Message::CreateStringTable(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_UpdateStringTable) => Message::UpdateStringTable(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_VoiceInit) => Message::VoiceInit(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_VoiceData) => Message::VoiceData(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_Print) => Message::Print(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_Sounds) => Message::Sounds(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_SetView) => Message::SetView(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_FixAngle) => Message::FixAngle(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_CrosshairAngle) => Message::CrosshairAngle(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_BSPDecal) => Message::BSPDecal(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_UserMessage) => Message::UserMessage(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_GameEvent) => Message::GameEvent(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_PacketEntities) => Message::PacketEntities(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_TempEntities) => Message::TempEntities(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_Prefetch) => Message::Prefetch(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_Menu) => Message::Menu(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_GameEventList) => Message::GameEventList(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_GetCvarValue) => Message::GetCvarValue(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_SplitScreen) => Message::SplitScreen(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_EntityMessage) => Message::EntityMessage(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_PaintmapData) => Message::PaintmapData(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_CmdKeyValues) => Message::CmdKeyValues(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_EncryptedData) => Message::EncryptedData(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_HltvReplay) => Message::HltvReplay(make(data, length)?),
            MessageType::Svc(SVC_Messages::svc_Broadcast_Command) => Message::BroadcastCommand(make(data, length)?),
        };
        Ok(message)
    }
}
