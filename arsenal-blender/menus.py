import bpy

from . import arsenal

blender_classes = []

# We don't have any menus yet

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)
