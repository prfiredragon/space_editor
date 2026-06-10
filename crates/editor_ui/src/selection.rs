use std::ops::ControlFlow::Continue;

use crate::*;
use bevy::prelude::*;
//use bevy::{color::palettes::tailwind::{PINK_100, RED_500}, picking::pointer::PointerInteraction, prelude::*};


/* #[derive(Resource, Default, Debug)]
pub struct HoveredMesh(pub Option<Entity>);
 */

/// This event used for selecting entities
#[derive(EntityEvent, Clone)]
#[entity_event(propagate)]
pub struct SelectEvent {
    entity: Entity
}


#[cfg(not(tarpaulin_include))]
pub fn plugin(app: &mut App) {
    if !app.is_plugin_added::<MeshPickingPlugin>() {
        app.add_plugins(MeshPickingPlugin);
    }

    //app.init_resource::<HoveredMesh>();

    //app.add_observer(on_pointer_click_b);
    //app.add_observer(on_pointer_click_c);


    //app.add_systems(
    //    Update,
    //    (delete_selected, reemit_pointer_click)// auto_add_markers)
    //);
    app.add_systems(
        Update,
        (auto_add_markers,
        //handle_click_selection,
    
    )
    );

    

    //app.add_systems(
    //    Update,
    //    draw_mesh_intersections.run_if(in_state(EditorState::Editor))
    //);

    //app.add_event::<AddMarkersEvent>();

    //app.add_observer(select_listener);
    //app.add_observer(recursive_add_markers);

    app.insert_resource(MeshPickingSettings {
        require_markers: false,
        ray_cast_visibility: RayCastVisibility::VisibleInView
    });
}


/* 
pub fn handle_click_selection(
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    hovered: Res<HoveredMesh>,
    query_selected: Query<Entity, With<Selected>>,
    mut commands: Commands,
) {
    // Solo reaccionamos en el instante del clic izquierdo
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    info!("Hovered : {:?}", hovered.0);
    // Si el ratón no está encima de ninguna malla 3D, ignoramos el clic
    let Some(target_entity) = hovered.0 else {
        return;
    };

    let is_shifting = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);

    if !is_shifting {
        // Limpiamos la selección anterior
        for e in query_selected.iter() {
            if e != target_entity {
                commands.entity(e).remove::<Selected>();
            }
        }
        commands.entity(target_entity).insert(Selected);
    } else {
        // Toggle con Shift
        if query_selected.contains(target_entity) {
            commands.entity(target_entity).remove::<Selected>();
        } else {
            commands.entity(target_entity).insert(Selected);
        }
    }
    
    info!("¡Selección perfecta vía Hover+Click!: {:?}", target_entity);
}

 */

/* fn auto_add_markers(
    mut commands: Commands,
    q_prefabs: Query<Entity, (With<PrefabMarker>, Without<MeshPickingCamera>)>,
    q_cameras: Query<Entity, (With<Camera3d>, Without<MeshPickingCamera>)>,
) {
    for entity in q_prefabs.iter() {
        commands.trigger(AddMarkersEvent { entity } );
    }

    for entity in q_cameras.iter() {
        commands.entity(entity).insert(MeshPickingCamera);
    }
} */

/* #[derive(EntityEvent, Clone)]
struct AddMarkersEvent{
    entity: Entity
}

fn recursive_add_markers(
    trigger: On<AddMarkersEvent>,
    q_children: Query<&Children>,
    q_meshes: Query<Entity, With<Mesh3d>>,
    mut commands: Commands,
) {
    if q_meshes.contains(trigger.entity) {
        commands.entity(trigger.entity).insert(Pickable {
            should_block_lower: true,
            is_hoverable: true,
        });
    }

    if let Ok(children) = q_children.get(trigger.entity) {
        for child in children.iter() {
            commands.trigger(AddMarkersEvent { entity: child.entity() } );
        }
    }
} */
/* 
/// From bevy examples
/// A system that draws hit indicators for every pointer.
fn draw_mesh_intersections(pointers: Query<&PointerInteraction>, mut gizmos: Gizmos) {
    for (point, normal) in pointers
        .iter()
        .filter_map(|interaction| interaction.get_nearest_hit())
        .filter_map(|(_entity, hit)| hit.position.zip(hit.normal))
    {
        gizmos.sphere(point, 0.05, RED_500);
        gizmos.arrow(point, point + normal.normalize() * 0.5, PINK_100);
    }
}

/// Reemits the pointer click event to the entity that is being clicked on
/// Its not a good solution, but it works for now
fn reemit_pointer_click(
    mut local: Local<bool>,
    pointers: Query<&PointerInteraction>,
    mut commands: Commands,
    q_meshes: Query<Entity, With<Mesh3d>>,
) {
    for pointer in pointers.iter() {
        if let Some((e, _)) = pointer.get_nearest_hit() {
            if q_meshes.contains(*e) {
                if !*local {
                    commands.trigger(SelectEvent {entity: *e});
                    *local = true;
                    return;
                } else {
                    // We will not reemit the event if it was already emitted in previous frame
                    return;
                }
            }
        }
    }

    // Clear the continuous flag
    *local = false;
} */

pub fn select_listener(
    mut trigger: On<SelectEvent>,
    mut commands: Commands,
    query: Query<Entity, With<Selected>>,
    // may need to be optimized a bit so that there is less overlap
    prefabs: Query<Entity, With<PrefabMarker>>,
    parents: Query<&ChildOf>,
    pan_orbit_state: ResMut<EditorCameraEnabled>,
    keyboard: Res<ButtonInput<KeyCode>>,
    //gizmo_query: Query<&GizmoTarget>,
) {
    //if gizmo_query.iter().any(|gizmo| gizmo.is_active()) {
    //    return;
    //}

    if !pan_orbit_state.0 {
        trigger.propagate(false);
        return;
    }

    info!("Select Event: {:?}", trigger.entity);

    if let Ok(entity) = prefabs.get(trigger.entity) {
        commands.entity(entity).insert(Selected);
        if !keyboard.pressed(KeyCode::ShiftLeft) {
            for e in query.iter() {
                commands.entity(e).remove::<Selected>();
            }
        }
    } else if let Ok(parent) = parents.get(trigger.entity) {
        // Just stupid propagation (Need to make it with Event trait)
        commands.trigger(SelectEvent {entity: parent.parent()}); 
    }
}






pub fn delete_selected(
    mut commands: Commands,
    query: Query<Entity, With<Selected>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let shift = keyboard.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = keyboard.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    let delete = keyboard.any_just_pressed([KeyCode::Backspace, KeyCode::Delete]);

    if ctrl && shift && delete {
        for entity in query.iter() {
            info!("Delete Entity: {entity:?}");
            commands.entity(entity).despawn();
        }
    }
}


/* pub fn on_pointer_click(
    _trigger: On<Pointer<Press>>,
    _commands: Commands,
    _q_meshes: Query<Entity, With<Mesh3d>>,
) {
    // info!("Pointer Click: {:?}", trigger.target());

    // if q_meshes.contains(trigger.target()) {
    //     commands.trigger_targets(SelectEvent, trigger.target());
    // }
} */

// Necesitamos importar KeyCode y ButtonInput para manejar la tecla Shift
use bevy::input::ButtonInput;

pub fn on_pointer_click(
    mut trigger: On<Pointer<Out>>, // Usamos Click para que sea más preciso que Over
    mut commands: Commands,
    query_selected: Query<Entity, With<Selected>>,
    query_cameras: Query<Entity, Or<(With<Camera3d>, With<PlaymodeCamera>)>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    info!("Pointer Over: {:?}", trigger.event_target());
    let target_entity = trigger.event_target();

    // 1. Filtro de seguridad: Ignorar cámaras
    if query_cameras.contains(target_entity) {
        return;
    }

    // 2. Lógica de Selección
    let is_shifting = keyboard.any_pressed([KeyCode::ShiftLeft]);
    let is_ctrling = keyboard.any_pressed([KeyCode::ControlLeft]);

    if !is_shifting & is_ctrling{
        // Modo selección única: Limpiar todo lo previo
        for e in query_selected.iter() {
            if e != target_entity {
                commands.entity(e).remove::<Selected>();
            }
        }
        commands.entity(target_entity).insert(Selected);
    } else if is_shifting {
        // Modo selección múltiple (Shift): Toggle (quitar si ya está, poner si no)
        if query_selected.contains(target_entity) {
            commands.entity(target_entity).remove::<Selected>();
        } else {
            commands.entity(target_entity).insert(Selected);
        }
    }

    trigger.propagate(true);
    //info!("Entidad seleccionada: {:?}", target_entity);
}


fn auto_add_markers(
    mut commands: Commands,
    q_prefabs: Query<Entity, (With<PrefabMarker>, With<Mesh3d>, Without<Pickable>)>,
    //q_cameras: Query<Entity, (With<Camera3d>, Without<MeshPickingCamera>)>,
    q_meshes: Query<Entity, (With<Mesh3d>, Without<Pickable>)>,
) {
    /* if !q_cameras.is_empty() {
        info!("q_cameras: {:?}", q_cameras);
        for entity in q_cameras.iter() {
            commands.entity(entity).insert(MeshPickingCamera);
        } 
    } */
    if q_prefabs.is_empty() {return;}
    info!("q_meshes: {:?}", q_meshes);
    info!("q_prefabs: {:?}", q_prefabs);
    /* for entity in q_prefabs.iter() {
        commands.trigger(AddMarkersEvent { entity } );
    } */

    /* for entity in q_cameras.iter() {
        commands.entity(entity).insert(MeshPickingCamera);
    } */
    for entity in q_prefabs.iter() {
        commands.entity(entity).insert(Pickable {
            should_block_lower: true,
            is_hoverable: true,
        }).observe(on_pointer_click)
        //.observe(on_pointer_click_over)
        // Cuando el ratón sale, limpiamos la memoria
        //.observe(on_pointer_click_out)
        
        ;
    }
}
/* 
pub fn on_pointer_click_over(trigger: On<Pointer<Over>>, mut hovered: ResMut<HoveredMesh>
){
    hovered.0 = Some(trigger.event_target());
    info!("Pointer Over: {:?}", trigger.event_target());
}

pub fn on_pointer_click_out(trigger: On<Pointer<Out>>, mut hovered: ResMut<HoveredMesh>
){
    info!("Pointer Out: {:?}", trigger.event_target());
    if hovered.0 == Some(trigger.event_target()) {
        hovered.0 = None;
    }
}

pub fn on_pointer_click_press(trigger: On<Pointer<Press>>, mut hovered: ResMut<HoveredMesh>
){
    //hovered.0 = Some(trigger.event_target());
    info!("Pointer Press: {:?}", trigger.event_target());
}

pub fn on_pointer_click_test(
    trigger: On<Pointer<Press>>,
    _commands: Commands,
    _q_meshes: Query<Entity, With<Mesh3d>>,
) {
     info!("Pointer Click: {:?}", trigger.event_target());

     /* if q_meshes.contains(trigger.target()) {
         commands.trigger_targets(SelectEvent, trigger.target());
     } */
}

pub fn on_pointer_click_b(
    mut trigger: On<Pointer<Press>>,
    _commands: Commands,
    _q_meshes: Query<Entity, With<Mesh3d>>,
    mut pan_orbit_state: ResMut<EditorCameraEnabled>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>,
) {
     info!("Pointer Press: {:?}", trigger.event_target());
     /* if !pan_orbit_state.0 {
        pan_orbit_state.0 = true;
        for mut pan_orbit in pan_orbit_query.iter_mut() {
            pan_orbit.enabled = false;
        }
        trigger.propagate(true);
        return;
    } */

     /* if q_meshes.contains(trigger.target()) {
         commands.trigger_targets(SelectEvent, trigger.target());
     } */
}

pub fn on_pointer_click_c(
    mut trigger: On<Pointer<Click>>,
    _commands: Commands,
    _q_meshes: Query<Entity, With<Mesh3d>>,
    mut pan_orbit_state: ResMut<EditorCameraEnabled>,
) {
     info!("Pointer Click: {:?}", trigger.event_target());
     /* if !pan_orbit_state.0 {
        pan_orbit_state.0 = true;
        trigger.propagate(true);
        return;
    } */

     /* if q_meshes.contains(trigger.target()) {
         commands.trigger_targets(SelectEvent, trigger.target());
     } */
} */