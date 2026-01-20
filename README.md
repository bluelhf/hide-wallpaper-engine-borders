# hide-wallpaper-engine-borders

**hide-wallpaper-engine-borders** is a small utility for X11 that hides the blue glow around the Wallpaper Engine GUI. Since Wine renders these as override-redirect windows, they are not managed by the window manager and render improperly.

> [!Note]
> For example, when switching to a tag that does not hold the wallpaper engine window, the borders still render as a stale overlay on the tag.

The tool runs as a daemon that automatically unmaps the windows without killing the respective process. More specifically, it unmaps any window that:
1. Does not have a title
2. Has an aspect ratio less than 1/3 or greater than 3/1.
3. Has a`WM_CLASS` property starting with `steam_app_431960<NULL>`

This should nearly always catch the borders and unmap them, while still allowing for the regular UI to be mapped by X11 normally.