# CSGO Demo Parser

Parses `.dem` files that are given from Counter-Strike: Global Offensive.

# Protobufs

I got the main protobufs from The [CSGO Game Tracker](https://github.com/SteamDatabase/GameTracking-CSGO),
and found the google protobuf file [here](https://github.com/ValvePython/csgo), but theres probably an updated version elsewhere.

## TODO:

- Move to [binrw](https://binrw.rs/) for binary parsing? Not sure how it will interact with Protobufs.
- Move from Anyhow to Thiserror
- Rustfmt config
- Check out the API of [this library](https://github.com/Alpha1337k/csgo-demoparser)
- Add license file
- Add a license file for the Valve .proto files...?
- We must use bytes for the protobufs, so we should probably get rid of ByteOrder.
- no_std support. Ez with byteorder, More work to ensure it works with quick-protobuf.
