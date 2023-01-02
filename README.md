# CSGO Demo Parser

Parses `.dem` files that are given from Counter-Strike: Global Offensive.

# Protobufs

I got the main protobufs from The [CSGO Game Tracker](https://github.com/SteamDatabase/GameTracking-CSGO),
and found the google protobuf file [here](https://github.com/ValvePython/csgo), but theres probably an updated version elsewhere.

# Usage

Get a .dem file. This can be from your own CSGO game (search for how to download these), or from the internet. [Here is a RAR of a game for testing (from HLTV)](https://www.hltv.org/download/demo/75565).
Take that (unrar it if it is a RAR file.), and run:

```
$ cargo run --release -- -o OUT.json /path/to/demofile.dem
```

# License

This is available for OSS purposes as GPLv3 code. This means that if you use or modify the code and distribute it, you must distribute your version of this code as well.

If you want to use this library for commercial purposes, please contact me, and we can come up with a deal.

The protobuf files in the `protobufs/` directory are from Valve Software. See `LICENSE.valve.md` for more information.

## TODO:

- Move from Anyhow to Thiserror?
- no_std support. Probably unlikely because Bitbuffer requires ownership sometimes.
- Releasing binaries on tag push.
- output useful information lol...

### TODO: Useful Information

Right now, we just output a JSON dump of the Demo file (minus some seemingly useless parts that just add cruft to the format).

What I want is to output concretely useful things, such as player positioning over time, and stats. I want this in a tabular format, for easy analysis.
