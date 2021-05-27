use bevy::prelude::*;
mod calc;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "bevy calculator".to_string(),
            width: 450.0,
            height: 600.0,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .insert_resource(calc::Calc::new())
        .add_startup_system(setup_calc_ui.system())
        .add_system(button_system.system())
        .add_system(display_system.system())
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
    cc: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.09, 0.09, 0.09).into()),
            pressed: materials.add(Color::rgb(0.7, 0.4, 0.0).into()),
            cc: materials.add(Color::rgb(0.7, 0.0, 0.0).into())
        }
    }
}

fn button_system(
    mut calc:ResMut<calc::Calc>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                button_press(&mut calc,text.sections[0].value.clone());
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
                if text.sections[0].value.to_string() == "C".to_string(){
                    *material = button_materials.cc.clone();
                }
            }
        }
   
    }
}

#[derive(Debug)]
struct DisplayText;

fn button_press(calc:&mut calc::Calc,val:String){
    match &val[..] {
        "0" => calc.add_display(0.0),
        "1" => calc.add_display(1.0),
        "2" => calc.add_display(2.0),
        "3" => calc.add_display(3.0),
        "4" => calc.add_display(4.0),
        "5" => calc.add_display(5.0),
        "6" => calc.add_display(6.0),
        "7" => calc.add_display(7.0),
        "8" => calc.add_display(8.0),
        "9" => calc.add_display(9.0),
        "+" => calc.add_symbol("+".to_string()),
        "-" => calc.add_symbol("-".to_string()),
        "*" => calc.add_symbol("*".to_string()),
        "/" => calc.add_symbol("/".to_string()),
        "=" => {
            let sym = calc.symbol();
            match &sym[..] {
                "+" => calc.add(),
                "-" => calc.sub(),
                "*" => calc.mult(),
                "/" => calc.div(),
                _ => return,
            }
        }
        "C" => calc.reset(),
        _ => return,
    }
}

fn display_system(calc:Res<calc::Calc>,mut query:Query<(&DisplayText,&mut Text)>){
        
        for (mut i,mut text) in query.iter_mut(){
            text.sections[0].value = calc.display();
        }
    
}
fn setup_calc_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn_bundle(UiCameraBundle::default());

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
        })
        .with_children(|parent| {
            parent
                // display (border)
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                        position_type: PositionType::Absolute,
                        position: Rect{left:Val::Px(0.0),top:Val::Px(0.0),..Default::default()},
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.4, 0.4, 0.4).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // display fill (content)
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::Baseline,
                                justify_content: JustifyContent::FlexEnd,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent
                            .spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section("0".to_string(), TextStyle {
                                    font: asset_server
                                    .load("fonts/FiraSans-Bold.ttf"),
                                    font_size:60.0,
                                    color: Color::WHITE,
                                }, TextAlignment{
                                    ..Default::default()
                                }),
                                    ..Default::default()
                                })
                                .insert( DisplayText);
                        });
                });
                // button panel
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                        flex_direction:FlexDirection::Row,
                        flex_wrap:FlexWrap::Wrap,
                        align_self:AlignSelf::FlexStart,
                        justify_content:JustifyContent::SpaceAround,
                        align_content:AlignContent::SpaceAround,
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.2, 0.2, 0.2).into()),
                    ..Default::default()
                })
                .with_children(|parent|{
                    let btnSymbols = 
                    vec![
                    "0","*","/","=",
                    "1","2","3","+",
                    "4","5","6","-",
                    "7","8","9","C"
                    ];
                    for i in btnSymbols{
                        parent
                        .spawn_bundle(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(110.5), Val::Px(103.0)),
                                // center button
                                
                                padding:Rect::all(Val::Px(2.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            material: button_materials.normal.clone(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                            .spawn_bundle(TextBundle {
                                text: Text::with_section(i.to_string(), TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                }, TextAlignment::default()),
                                ..Default::default()
                            });
                        });
                    }
                });
        });
}
