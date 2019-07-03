import bpy

blender_classes = []

class ArsenalRunner(bpy.types.Panel):
    """Arsenal Runner"""
    bl_label = "Arsenal"
    bl_idname = "ARSENAL_PT_runner"
    bl_space_type = 'PROPERTIES'
    bl_region_type = 'WINDOW'
    bl_context = "render"

    def draw(self, context):
        layout = self.layout

        row = layout.row()
        row.operator("arsenal.run", icon='PLAY')

blender_classes.append(ArsenalRunner)

def register():
    for blender_class in blender_classes:
        bpy.utils.register_class(blender_class)

def unregister():
    blender_classes.reverse()
    for blender_class in blender_classes:
        bpy.utils.unregister_class(blender_class)
