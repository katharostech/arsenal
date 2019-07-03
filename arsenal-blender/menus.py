import bpy

from . import arsenal

blender_classes = []

class SaveBeforeRunMenu(bpy.types.Menu):
    """Menu displayed when attempting to run an unsaved blend"""
    bl_label = "Save Blend Before Running Game"
    bl_idname = "ARSENAL_MT_save_before_run"

    def draw(self, context):
        layout = self.layout
        layout.operator_context = 'INVOKE_AREA'

        layout.operator("wm.save_mainfile")


blender_classes.append(SaveBeforeRunMenu)

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)
