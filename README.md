# Spectre Sidecar
This repo contains the sidecar application for [Spectre](https://svrc.app). The sidecar is a small app that runs in the background, when game logging is enabled in Spectre. It's primary function is to read VRChat game logs and return errors and other messages over a websocket. Spectre can then use these messages to display notifications, create a game log view, and perform other tasks.

This sidecar could also be used by any other app to perform similar functions over websocket IPC.

RegEx can be modified in the `regex.rs` module file, and websocket messages are sent over port 40602 by default. The port can be changed by launching with `-p <port>` and the default location for logs can be set with the `-d <directory` flag. By default, the sidecar looks for the latest output_log_*.txt file in the `C:\\Users\\%USERNAME%\\Application Data\\LocalLow\\VRChat` directory.

Please note that precompiled versions of this binary are included in the [Spectre repository](https://github.com/angelware-net/spectre), and are built into the Tauri application bundles.
