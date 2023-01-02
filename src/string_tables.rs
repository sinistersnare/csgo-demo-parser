use std::collections::HashMap;

use serde::Serialize;

use crate::cursor::Cursor;

#[derive(Debug, Serialize)]
pub struct StringTables {
    tables: Vec<StringTable>,
}

impl StringTables {
    pub fn parse(chunk: &Cursor) -> anyhow::Result<StringTables> {
        let num_tables = chunk.read_u8()?;
        let mut tables = Vec::with_capacity(num_tables as usize);
        for _ in 0..num_tables {
            let table_name = chunk.read_cstr_until()?;
            match table_name.as_ref() {
                "userinfo" => tables.push(StringTable::UserInfo(PlayerInfo::parse(chunk)?)),
                "instancebaseline" => {
                    let num_strings = chunk.read_u16()?;
                    let mut mapping = HashMap::with_capacity(num_strings as usize);
                    for _ in 0..num_strings {
                        let string_name = chunk.read_cstr_until()?;
                        if chunk.read_bit_bool()? {
                            let user_data_size = chunk.read_u16()?;
                            let data = chunk.read_bytes(user_data_size as usize)?.into_owned();
                            let id = string_name.parse()?;
                            mapping.insert(id, data);
                        }
                    }
                    tables.push(StringTable::InstanceBaseline(mapping))
                }
                "modelprecache" => {
                    let num_strings = chunk.read_u16()?;
                    let mut models = Vec::with_capacity(num_strings as usize);
                    for _ in 0..num_strings {
                        let string_name = chunk.read_cstr_until()?.into_owned();
                        if string_name.len() > 100 {
                            anyhow::bail!("MODEL NAME TOO LONG! {}", string_name.len());
                        }
                        if chunk.read_bit_bool()? {
                            let user_data_size = chunk.read_u16()?;
                            if user_data_size != 0 {
                                anyhow::bail!("User data in model precache table.");
                            }
                            models.push(string_name);
                        }
                    }
                    tables.push(StringTable::ModelPrecache(models))
                }
                unknown => {
                    anyhow::bail!("Unknown table name `{unknown}`.")
                }
            }
        }
        Ok(StringTables { tables })
    }
}

#[derive(Debug, Serialize)]
pub enum StringTable {
    UserInfo(HashMap<u8, PlayerInfo>),
    InstanceBaseline(HashMap<i32, Vec<u8>>),
    ModelPrecache(Vec<String>),
}

// TODO: why cant these Strings be &'a str :'(
#[derive(Debug, Serialize)]
pub struct PlayerInfo {
    version: i64,
    xuid: i64,
    name: String,
    user_id: i32,
    guid: String,
    friends_id: i32,
    friends_name: String,
    is_fake_player: bool,
    is_hltv: bool,
    custom_files: [i32; 4],
    files_downloaded: u8,
}

impl PlayerInfo {
    pub fn parse(chunk: &Cursor) -> anyhow::Result<HashMap<u8, PlayerInfo>> {
        let mut mapping = HashMap::new();
        let num_strings = chunk.read_u16()?;
        for _ in 0..num_strings {
            let string_name = chunk.read_cstr_until()?;
            let which_player: u8 = string_name.parse()?;
            if chunk.read_bit_bool()? {
                let user_data_size = chunk.read_i16()?;
                let info_chunk = chunk.chunk_bytes(user_data_size as usize)?;
                let version = info_chunk.read_i64()?;
                let xuid = info_chunk.read_i64()?;
                let name = info_chunk.read_cstr(128)?.into_owned();
                let user_id = info_chunk.read_i32()?;
                let guid = info_chunk.read_cstr(33)?.into_owned();
                let friends_id = info_chunk.read_i32()?;
                let friends_name = info_chunk.read_cstr(128)?.into_owned();
                let is_fake_player = info_chunk.read_byte_bool()?;
                let is_hltv = info_chunk.read_byte_bool()?;
                let cf1 = info_chunk.read_i32()?;
                let cf2 = info_chunk.read_i32()?;
                let cf3 = info_chunk.read_i32()?;
                let cf4 = info_chunk.read_i32()?;
                let custom_files = [cf1, cf2, cf3, cf4];
                let files_downloaded = info_chunk.read_u8()?;
                let player_info = PlayerInfo {
                    version,
                    xuid,
                    name,
                    user_id,
                    guid,
                    friends_id,
                    friends_name,
                    is_fake_player,
                    is_hltv,
                    custom_files,
                    files_downloaded,
                };
                mapping.insert(which_player, player_info);
            }
        }
        Ok(mapping)
    }
}
