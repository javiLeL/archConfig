#   ▄▄▄██▀▀▀ ▄▄▄       ██▒   █▓ ██▓      ██▓    ▓█████  ██▓    
#     ▒██   ▒████▄    ▓██░   █▒▓██▒     ▓██▒    ▓█   ▀ ▓██▒    
#     ░██   ▒██  ▀█▄   ▓██  █▒░▒██▒     ▒██░    ▒███   ▒██░    
#  ▓██▄██▓  ░██▄▄▄▄██   ▒██ █░░░██░     ▒██░    ▒▓█  ▄ ▒██░    
#   ▓███▒    ▓█   ▓██▒   ▒▀█░  ░██░ ██▓ ░██████▒░▒████▒░██████▒
#   ▒▓▒▒░    ▒▒   ▓▒█░   ░ ▐░  ░▓   ▒▓▒ ░ ▒░▓  ░░░ ▒░ ░░ ▒░▓  ░
#   ▒ ░▒░     ▒   ▒▒ ░   ░ ░░   ▒ ░ ░▒  ░ ░ ▒  ░ ░ ░  ░░ ░ ▒  ░
#   ░ ░ ░     ░   ▒        ░░   ▒ ░ ░     ░ ░      ░     ░ ░   
#   ░   ░         ░  ░      ░   ░    ░      ░  ░   ░  ░    ░  ░
#                          ░         ░                         
#    █████  ▄▄▄█████▓ ██▓ ██▓    ▓█████                        
#  ▒██▓  ██▒▓  ██▒ ▓▒▓██▒▓██▒    ▓█   ▀                        
#  ▒██▒  ██░▒ ▓██░ ▒░▒██▒▒██░    ▒███                          
#  ░██  █▀ ░░ ▓██▓ ░ ░██░▒██░    ▒▓█  ▄                        
#  ░▒███▒█▄   ▒██▒ ░ ░██░░██████▒░▒████▒                       
#  ░░ ▒▒░ ▒   ▒ ░░   ░▓  ░ ▒░▓  ░░░ ▒░ ░                       
#   ░ ▒░  ░     ░     ▒ ░░ ░ ▒  ░ ░ ░  ░                       
#     ░   ░   ░       ▒ ░  ░ ░      ░                          
#      ░              ░      ░  ░   ░  ░                       
#                                                              

from libqtile import bar, layout, widget, qtile, hook
from libqtile.config import Click, Drag, Group, Key, Match, Screen
from libqtile.lazy import lazy
from qtile_extras import widget
from qtile_extras.widget.decorations import PowerLineDecoration

import os
import subprocess

mod = "mod4"
homefolder = os.path.expanduser('~')

# Powerline decorations
powerline = {"decorations": [PowerLineDecoration(path='arrow_right')]}
powerlineLeft = {"decorations": [PowerLineDecoration(path='arrow_left')]}
powerlineRoundLeft = {"decorations": [PowerLineDecoration(path='rounded_left')]}
powerlineRoundRight = {"decorations": [PowerLineDecoration(path='rounded_right')]}
powerlineBack = {"decorations": [PowerLineDecoration(path='back_slash')]}

keys = [
    # A list of available commands that can be bound to keys can be found
    # at https://docs.qtile.org/en/latest/manual/config/lazy.html
    # Switch between windows
    Key([mod], "h", lazy.layout.left(), desc="Move focus to left"),
    Key([mod], "l", lazy.layout.right(), desc="Move focus to right"),
    Key([mod], "j", lazy.layout.down(), desc="Move focus down"),
    Key([mod], "k", lazy.layout.up(), desc="Move focus up"),
    Key([mod], "space", lazy.layout.next(), desc="Move window focus to other window"),
    # Move windows between left/right columns or move up/down in current stack.
    # Moving out of range in Columns layout will create new column.
    Key([mod, "shift"], "h", lazy.layout.shuffle_left(), desc="Move window to the left"),
    Key([mod, "shift"], "l", lazy.layout.shuffle_right(), desc="Move window to the right"),
    Key([mod, "shift"], "j", lazy.layout.shuffle_down(), desc="Move window down"),
    Key([mod, "shift"], "k", lazy.layout.shuffle_up(), desc="Move window up"),
    # Grow windows. If current window is on the edge of screen and direction
    # will be to screen edge - window would shrink.
    Key([mod, "control"], "h", lazy.layout.grow_left(), desc="Grow window to the left"),
    Key([mod, "control"], "l", lazy.layout.grow_right(), desc="Grow window to the right"),
    Key([mod, "control"], "j", lazy.layout.grow_down(), desc="Grow window down"),
    Key([mod, "control"], "k", lazy.layout.grow_up(), desc="Grow window up"),
    Key([mod], "n", lazy.layout.normalize(), desc="Reset all window sizes"),

    # Toggle between split and unsplit sides of stack.
    # Split = all windows displayed
    # Unsplit = 1 window displayed, like Max layout, but still with
    # multiple stack panes
    Key(
        [mod, "shift"],
        "Return",
        lazy.layout.toggle_split(),
        desc="Toggle between split and unsplit sides of stack",
    ),
    Key([mod], "Return", lazy.spawn("alacritty"), desc="Launch terminal"),

    # Toggle between different layouts as defined below
    Key([mod], "Tab", lazy.next_layout(), desc="Toggle between layouts"),
    Key([mod], "w", lazy.window.kill(), desc="Kill focused window"),
    Key([mod, "control"], "r", lazy.reload_config(), desc="Reload the config"),
    Key([mod, "control"], "q", lazy.shutdown(), desc="Shutdown Qtile"),
    Key([mod], "r", lazy.spawncmd(), desc="Spawn a command using a prompt widget"),

    # My binds
    Key([mod], "s", lazy.spawn("firefox"), desc="Open Firefox"),
    Key([mod, "control"], "Return", lazy.spawn("rofi -show drun"), desc="Menu rofi"),
    
    # Brigthnes
    # Key([], "XF86MonBrightnessUp", lazy.spawn("brightnessctl set +5%"), desc="More brightnes"),
    # Key([], "XF86MonBrightnessDown", lazy.spawn("brightnessctl set 5%-"), desc="Less brightnes"),

    Key([], "XF86MonBrightnessUp", lazy.spawn(homefolder+"/.config/qtile/myScripts/brightness up"), desc="More brightnes"),
    Key([], "XF86MonBrightnessDown", lazy.spawn(homefolder+"/.config/qtile/myScripts/brightness down"), desc="Less brightnes"),
    Key([mod], "F6", lazy.spawn("brightnessctl set 0"), desc="Null brightnes"),
    
    # window to laout
    Key([mod, "control"], "n", lazy.window.toggle_floating(), desc="Put the windows in layouts"),
    
    # Volume
    # Whitout notification
    # Key([], "XF86AudioLowerVolume", lazy.spawn("pamixer -d 5"), desc="Low the volume"),
    # Key([], "XF86AudioRaiseVolume", lazy.spawn("pamixer -i 5"), desc="Up the volume"),
    # Key([], "XF86AudioMute", lazy.spawn("pamixer -t"), desc="Toggle the mute and unmute"),
    
    # With notification
    Key([], "XF86AudioRaiseVolume", lazy.spawn(homefolder+"/.config/qtile/myScripts/volumne up"), desc="Up the volume"),
    Key([], "XF86AudioLowerVolume", lazy.spawn(homefolder+"/.config/qtile/myScripts/volumne down"), desc="Low the volume"),
    Key([], "XF86AudioMute", lazy.spawn(homefolder+"/.config/qtile/myScripts/volumne mute"), desc="Toggle the mute and unmute"),

    # TouchPad ON OFF
    Key([mod], "F10", lazy.spawn(homefolder+"/.config/qtile/myScripts/trackpad-toggle"), desc="Toggle the touchpad on/off"),
    Key([], "XF86TouchpadToggle", lazy.spawn(homefolder+"/.config/qtile/myScripts/trackpad-toggle"), desc="Toggle the touchpad"),
    # Key([], "Super_L", lazy.spawn("arandr"), desc="Arandr"),
]

groups = [Group(i) for i in [
        "  ", "  ", "  ", "  ", "  ", "  "
]]

for i, group in enumerate(groups):
    numDesktop = str(i+1)
    keys.extend(
        [
            # mod1 + letter of group = switch to group
            Key(
                [mod],
                numDesktop,
                lazy.group[group.name].toscreen(),
                desc="Switch to group {}".format(group.name),
            ),
            # mod1 + shift + letter of group = switch to & move focused window to group
            Key(
                [mod, "shift"],
                numDesktop,
                lazy.window.togroup(group.name, switch_group=True),
                desc="Switch to & move focused window to group {}".format(group.name),
            ),
            # Or, use below if you prefer not to switch to that group.
            # # mod1 + shift + letter of group = move focused window to group
            # Key([mod, "shift"], i.name, lazy.window.togroup(i.name),
            #     desc="move focused window to group {}".format(i.name)),
        ]
    )

layouts = [
    layout.Columns(border_focus="00a900", border_normal="005000", border_width=3, margin=5),
    layout.Max(),
    # Try more layouts by unleashing below layouts.
    # layout.Stack(num_stacks=2),
    # layout.Bsp(),
    # layout.Matrix(),
    # layout.MonadTall(),
    # layout.MonadWide(),
    # layout.RatioTile(),
    # layout.Tile(),
    # layout.TreeTab(),
    # layout.VerticalTile(),
    # layout.Zoomy(),
]

widget_defaults = dict(
    # font="sans",
    font = 'Ubuntu Mono Font',
    fontsize=20,
    padding=4,
)
extension_defaults = widget_defaults.copy()

screens = [
    Screen(
        bottom=bar.Bar(
            [
                # widget.CurrentLayout(),
                
                widget.Image(
                    filename = "~/.local/share/icons/hicolor/32x32/raccon.png",
                    # background = "#000000",
                    # mouse_callbacks = {"Button1": lazy.spawncmd()},
                    # margin = 3,
                ),
                
                #widget.Sep(
                #    background="000000",
                #    linewidth = 10,
                #    size_percent = 1,
                #    **powerlineLeft
		        #    ),

                widget.GroupBox(
                    background = "000000",
                    this_current_screen_border = "00aa00",
                    this_screen_border = "00aa00",
                    highlight_color = ['000000', '005000'],
                    disable_drag = True,
                    highlight_method='line',
                    active="00a900",
                    inactive="005f00",
                    fontsize=20,
                    **powerlineLeft
                ),

                widget.TaskList(
                    border = "00a900",
                    unfocused_border = "005f00",
                    txt_floating= "   ",
                    txt_maximized="   ",
                    txt_minimized="   ",
                    rounded = False,
                    fontsize=15
                    ),
                # widget.WindowName(fontsize=16),
                # widget.Chord(chords_colors={"launch": ("#ff0000", "#ffffff"),},name_transform=lambda name: name.upper(),),
                # widget.TextBox("default config", name="default"),
                # widget.TextBox("Press &lt;M-r&gt; to spawn", foreground="#d75f5f"),

                widget.Sep(linewidth = 0, **powerline),
                widget.WidgetBox(
                    background="15af15",
        	        text_open = ' ',
                    text_closed = ' ',
                    widgets=[
                    widget.CPU(
                        background="159c15",
		                format = '  : {freq_current} GHz {load_percent:5.2f}%',
                        **powerline
                        ),
                    widget.Memory(
                        background="158a15",
                        measure_mem = 'G',
                        format = '  : {MemUsed: .0f}{mm}/{MemTotal: .0f}{mm} ',
                        mouse_callbacks = {'Button1': lambda: qtile.cmd_spawn('alacritty -e htop')},
                        **powerline
                        ),
                    widget.ThermalSensor(
                        background="157015",
                        format = ' : {temp:.1f}{unit}  ',
                        mouse_callbacks = {'Button1': lambda: qtile.cmd_spawn('alacritty -e watch sensors')},
                        **powerline
                        ),   
                    ],
                    **powerline
                    ),

                widget.Battery(
                    fontsize = 20,
                    low_percentage=.20,
                    low_foreground="ff0000",
                    background="155f15", 
                    charge_char='󰂄',
                    discharge_char='󰂌',
                    full_char='󰁹',
                    unknown_char='󰂑',
                    format='{char}  {percent:2.0%} {hour:d}:{min:02d}',
                    mouse_callbacks = {'Button3': lambda: qtile.cmd_spawn('alacritty -e sudo nvim /etc/tlp.conf')},
                    **powerline
                ),

                widget.Clock(
                    format = '  %H:%M    󰃭  %d/%m/%y',
                    background= "154915",
                    padding = 6, 
                    mouse_callbacks = {'Button1': lambda: qtile.cmd_spawn(homefolder+'/.config/qtile/myScripts/calendar2 curr'), 'Button5': lambda: qtile.cmd_spawn(homefolder+'/.config/qtile/myScripts/calendar2 next'),'Button4': lambda: qtile.cmd_spawn(homefolder+'/.config/qtile/myScripts/calendar2 prev')},
                    **powerline
                    ),

                widget.Prompt(
                        background = "153a15",
                        fontsize = 20,
                        **powerline
                    ),

                widget.Systray(
                    fontsize=16,
                    background="153015",
                    ),

                widget.Sep(
                    
                    background="153015",
                    linewidth = 10,
                    size_percent = 1,
		            ),

            ],
            30,
            # border_width=[2, 0, 2, 0],  # Draw top and bottom borders
            # border_color=["ff00ff", "000000", "ff00ff", "000000"]  # Borders are magenta
        ),
    ),
]

# Drag floating layouts.
mouse = [
    Drag([mod], "Button1", lazy.window.set_position_floating(), start=lazy.window.get_position()),
    Drag([mod], "Button3", lazy.window.set_size_floating(), start=lazy.window.get_size()),
    Click([mod], "Button2", lazy.window.bring_to_front()),
]

dgroups_key_binder = None
dgroups_app_rules = []  # type: list
follow_mouse_focus = True
bring_front_click = False
cursor_warp = False
floating_layout = layout.Floating(
    float_rules=[
        # Run the utility of `xprop` to see the wm class and name of an X client.
        *layout.Floating.default_float_rules,
        Match(wm_class="confirmreset"),  # gitk
        Match(wm_class="makebranch"),  # gitk
        Match(wm_class="maketag"),  # gitk
        Match(wm_class="ssh-askpass"),  # ssh-askpass
        Match(title="branchdialog"),  # gitk
        Match(title="pinentry"),  # GPG key password entry
    ]
)
auto_fullscreen = True
focus_on_window_activation = "smart"
reconfigure_screens = True

# If things like steam games want to auto-minimize themselves when losing
# focus, should we respect this or not?
auto_minimize = True

# When using the Wayland backend, this can be used to configure input devices.
wl_input_rules = None

# XXX: Gasp! We're lying here. In fact, nobody really uses or cares about this
# string besides java UI toolkits; you can see several discussions on the
# mailing lists, GitHub issues, and other WM documentation that suggest setting
# this string if your java app doesn't work correctly. We may as well just lie
# and say that we're a working one by default.
#
# We choose LG3D to maximize irony: it is a 3D non-reparenting WM written in
# java that happens to be on java's whitelist.
wmname = "Qtile"

@hook.subscribe.startup_once
def autostart():
    home = os.path.expanduser('~')
    subprocess.Popen([home + '/.config/qtile/autostart.sh'])
