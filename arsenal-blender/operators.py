import bpy

from . import arsenal

blender_classes = []

class ArsenalRun(bpy.types.Operator):
    """Run Arsenal Game"""
    bl_idname = "arsenal.run"
    bl_label = "Run Game"

    def execute(self, context):
        # Ensure blend has been saved before running game
        if bpy.data.filepath == "":
            def draw_popup(popup, context):
                popup.layout.operator_context = 'INVOKE_AREA'
                popup.layout.label(text="Save Blend Before Running Game")
                popup.layout.operator("wm.save_mainfile")
            context.window_manager.popover(draw_popup)
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
