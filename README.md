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

## Further explanation:

Before setting up this launcher, an application may work as follows:

`Browser -> Application w/ launch arguments`

After setting up this launcher, an application will work as follows:

`Browser ->
Launcher w/ launch arguments ->
Launch arguments are written to config file ->
Launcher tells Steam to run launcher (overlay and controller support enabled here) ->
Launcher reads arguments from config file, clears arguments from file ->
Application w/ launch arguments`

Note that the only reason the launcher runs the game the second time it's ran is because of
the `-steam_game_launch` argument, which tells it that Steam is launching it to play the game
rather than a browser or something else.
