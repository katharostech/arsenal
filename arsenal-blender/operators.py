import bpy

blender_classes = []

class ArsenalRun(bpy.types.Operator):
    """Run Arsenal Game"""
    bl_idname = "arsenal.run"
    bl_label = "Run Game"

    def execute(self,   context):
        print("Running Arsenal Game")
        return {'FINISHED'}

blender_classes.append(ArsenalRun)

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)
