use bevy::prelude::*;
mod calc;
/// This example illustrates how to create a button that changes color and text based on its interaction state.
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "bevy calculator".to_string(),
            width: 450,
            height: 600,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .init_resource::<ButtonMaterials>()
        .add_resource(calc::Calc::new())
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

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
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
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children) in &mut interaction_query.iter() {
        let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed;
                button_press(&mut calc,text.value.clone());
            }
            Interaction::Hovered => {
                *material = button_materials.hovered;
            }
            Interaction::None => {
                *material = button_materials.normal;
                if text.value.to_string() == "C".to_string(){
                    *material = button_materials.cc;
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
        
        for (mut i,mut text) in &mut query.iter(){
            text.value = calc.display();
        }
    
}
fn setup_calc_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
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
                .spawn(NodeComponents {
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
                        // left vertical fill (content)
                        .spawn(NodeComponents {
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
                            .spawn((DisplayText,))
                            .with_bundle(TextComponents {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text {
                                    value: "0".to_string(),
                                    font: asset_server
                                        .load("assets/fonts/FiraSans-Bold.ttf")
                                        .unwrap(),
                                    style: TextStyle {
                                        font_size:60.0,
                                        color: Color::WHITE,
                                    },
                                },
                                ..Default::default()
                            });
                        });
                })
                // button panel
                .spawn(NodeComponents {
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
                        .spawn(ButtonComponents {
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
                            material: button_materials.normal,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextComponents {
                                text: Text {
                                    value: i.to_string(),
                                    font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                                    style: TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.8, 0.8, 0.8),
                                    },
                                },
                                ..Default::default()
                            });
                        });
                    }
                });
        });
}
