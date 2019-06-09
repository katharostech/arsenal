bl_info = {
    "name": "Arsenal",
    "author": "Katharos Technology<katharostech.com>",
    "version": (0, 1, 0),
    "blender": (2, 80, 0),
    "location": "Everywhere",
    "description": "Arsenal Game Engine",
    "category": "Game",
}

import bpy
from . import operators
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
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

    operators.register()
    panels.register()

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)

    panels.unregister()
    operators.unregister()

