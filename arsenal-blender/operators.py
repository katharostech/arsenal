import bpy

from . import arsenal
from .menus import SaveBeforeRunMenu

blender_classes = []

class ArsenalRun(bpy.types.Operator):
    """Run Arsenal Game"""
    bl_idname = "arsenal.run"
    bl_label = "Run Game"

    def execute(self, context):
        # Ensure blend has been saved before running game
        if bpy.data.filepath == "":
            bpy.ops.wm.call_menu(name=SaveBeforeRunMenu.bl_idname)
            return {'FINISHED'}

        print("Running Arsenal Game")
        arsenal.operators.arsenal_run(context)
        return {'FINISHED'}

blender_classes.append(ArsenalRun)

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)
