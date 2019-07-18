import bpy

from .operators import ArsenalRun

key_maps = []

def register():
    wm = bpy.context.window_manager

    # Add F5 shortcut for running the game
    window_keymap = wm.keyconfigs.addon.keymaps.new(name='Window', space_type='EMPTY', region_type="WINDOW")
    window_keymap.keymap_items.new(ArsenalRun.bl_idname, type='F5', value='PRESS')
    key_maps.append(window_keymap)

def unregister():
    wm = bpy.context.window_manager
    for key_map in key_maps:
        wm.key_configs.addon.keymaps.remove(key_map)
    del key_maps[:]
