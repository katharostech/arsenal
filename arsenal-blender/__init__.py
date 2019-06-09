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

blender_classes = []

def register():
    operators.register()

def unregister():
    operators.unregister()

