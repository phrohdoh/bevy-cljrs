mod scripting;
mod units;

use std::{env, net::SocketAddr, sync::Arc};
use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, find_my_ip_address};
use cljrs::{environment::Environment as Env, repl::Repl};
use scripting::{
    ScriptableApp as _,
    ScriptingConfig,
    create_custom_cljrs_env,
};

pub fn build_app() -> AppBuilder {
    let mut app = App::build();
    app
       .insert_resource(WindowDescriptor {
           title: "bevy + cljrs".into(),
           width: 200.0,
           height: 200.0,
           ..Default::default()
       })
       .add_plugins(DefaultPlugins)
       .add_plugin(units::UnitPlugin)
       .add_scripting(create_custom_cljrs_env(), ScriptingConfig {
           startup_repl: env::args().nth(1) == Some("--with-startup-repl".to_owned()),
       })
       // networking lib
       .add_plugin(NetworkingPlugin::default())
       .add_startup_system(sys_setup_net_listen.system())
       // our networking
       .add_system(sys_on_packet_recv.system())
    ;
    app
}

fn sys_setup_net_listen(
    mut net: ResMut<NetworkResource>,
) {
    let ip_addr = dbg!(find_my_ip_address()).expect("can't get own IP");
    let ip_addr = dbg!(ip_addr);
    let ip_addr = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
    let ip_addr = dbg!(ip_addr);
    let server_addr = SocketAddr::new(ip_addr, 8080);
    let server_addr = dbg!(server_addr);
    net.listen(server_addr, None, None);
}

fn sys_on_packet_recv(
    mut net: ResMut<NetworkResource>,
    mut rdr: EventReader<NetworkEvent>,
    mut arc_env: Res<Arc<Env>>,
) {
    let repl = Repl::new(arc_env.clone());

    for evt in rdr.iter() {
        if let NetworkEvent::Packet(hndl, bytes) = evt {
            let content = String::from_utf8_lossy(bytes);
            repl.eval_readable(&mut content.as_bytes());
        }
    }
}
