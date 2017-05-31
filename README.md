# Steam Arguments Launcher

Launches applications through Steam. Arguments passed into the launcher are passed into the configured application.

This is intended for situations where an application is launched from another
environment e.g. a browser, where one has little-to-no control over how it gets
launched and what arguments are passed to it. By replacing the original executable,
this launcher captures the arguments, then launches the game through Steam.

## Use instructions

This launcher should be put in place of the original application, and be configured to launch the
(now renamed) application. Steam should be configured to have a non-steam game set to
this launcher, with "launch options" set to `-steam_game_launch`

The `gameid` of the non-steam game will be used when the configurator asks for
the game id. It can be retrieved by creating a desktop shortcut for the non-steam
game and checking its properties. The `gameid` is the numbers following `steam://rungameid/`

## Configuration options

* application name (`exe_name`): The file name of the renamed application
* steam path (`steam_path`): The path to Steam, e.g. `C:\Program Files (x86)\Steam\Steam.exe`
* game id (`steam_gameid`): The game id of this launcher for the specific application

Additionally, the `debug` option can be turned on by editing the configuration file.
This will display information when the game is launched leave the launch windows open.

The `args` setting is used internally between activating Steam and launching the
actual application. It is automatically set when activating Steam and cleared
when launching the application.
