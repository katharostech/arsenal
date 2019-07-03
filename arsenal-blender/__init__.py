bl_info = {
    "name": "Arsenal",
    "author": "Katharos Technology<katharostech.com>",
    "version": (0, 1, 0),
    "blender": (2, 80, 0),
    "location": "Everywhere",
    "description": "Arsenal Game Engine",
    "category": "Game",
}

from os import chmod
from os.path import dirname, join
import stat
from glob import glob

import bpy
from . import operators
from . import menus
from . import panels

blender_classes = []

class ArsenalAddonPreferences(bpy.types.AddonPreferences):
    # this must match the add-on name, use '__package__'
    # when defining this in a submodule of a python package.
    bl_idname = __name__

    display_unimplemented: bpy.props.BoolProperty(
        name="Display Unimplemented",
        description="Display unimplemented UI elements. By default only UI components that work are displayed",
        default=False,
    )

    def draw(self, context):
        layout = self.layout
        layout.prop(self, "display_unimplemented")

blender_classes.append(ArsenalAddonPreferences)

def register():
    # Ensure "Execute" permissions on files in the "bin" dir
    addon_dir = dirname(__spec__.origin)
    bin_path = join(addon_dir, "bin", "*")
    for file_path in glob(bin_path):
        chmod(file_path, 0o755)

    # Register Blender Classes
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

    operators.register()
    menus.register()
    panels.register()

def unregister():
    # Unregister Blender Classes
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)

    panels.unregister()
    menus.unregister()
    operators.unregister()

