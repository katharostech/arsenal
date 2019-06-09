import bpy

blender_classes = []

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    for blender_class in blender_classes.reverse():
        bpy.utils.unregister_class(blender_class)
