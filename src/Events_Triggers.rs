//demo on event works and triggers

use bevy::prelude::*;

#[derive(Event)]

struct DamageEvent{
    amount: u32,
}

impl Default for DamageEvent{ //traits: implementing a trait, basically setting a default amount, so we dont have to set it everytime its called
    fn default() -> Self {
        Self {amount: 0}
    }
}

struct ExplosionEvent;

fn main (){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_event::<DamageEvent>()
    .add_systems(Startup, (deal_damage, make_boom))//ANY add_system add .chain() make them go in order, use at your own risk
    .add_systems(Update, react_to_damage)
    .add_observer(on_explosion)
    .run();
}

fn deal_damage( //Added to cue 
    mut writer: EventWriter<DamageEvent>
){
    writer.send(DamageEvent{amount: 5}); //cause of impl, can put (DamageEvent::default());
}

fn react_to_damage( //read then CONSUME the event in the cue, so it cant get read again
    mut reader: EventReader<DamageEvent>
){
    for e in reader.read(){
        //Do a thing
        info!("Damage {}", e.amount);
    }

}

fn on_explosion( //this happens immediately, no cue, this is the recieve 
    trig: Trigger<ExplosionEvent>
){

    info!("EXPLODE!!!!");
}

fn make_boom(mut commands: Commands){ //this is the send

    commands.trigger(ExplosionEvent);
}
