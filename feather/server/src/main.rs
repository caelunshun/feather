use anyhow::Context;
use base::anvil::level::SuperflatGeneratorOptions;
use base::biome::{BiomeGeneratorInfo, BiomeList};
use base::world::DimensionInfo;
use common::world::{Dimensions, WorldName, WorldPath};
use common::{Dimension, Game, TickLoop};
use data_generators::extract_vanilla_data;
use ecs::SystemExecutor;
use feather_server::{config::Config, Server};
use plugin_host::PluginManager;
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc, sync::Arc};
use worldgen::{SuperflatWorldGenerator, WorldGenerator};

mod logging;

const PLUGINS_DIRECTORY: &str = "plugins";
const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let feather_server::config::ConfigContainer {
        config,
        was_config_created,
    } = feather_server::config::load(CONFIG_PATH).context("failed to load configuration file")?;
    logging::init(config.log.level);
    if was_config_created {
        log::info!("Created default config");
    }
    log::info!("Loaded config");

    extract_vanilla_data();

    log::info!("Creating server");
    let options = config.to_options();
    let server = Server::bind(options).await?;

    let mut game = init_game(server, &config)?;
    game.insert_resource(config);

    run(game);

    Ok(())
}

fn init_game(server: Server, config: &Config) -> anyhow::Result<Game> {
    let mut game = Game::new();
    init_systems(&mut game, server);
    init_biomes(&mut game);
    init_worlds(&mut game, config);
    init_dimensions(&mut game, config);
    init_plugin_manager(&mut game)?;
    Ok(game)
}

fn init_systems(game: &mut Game, server: Server) {
    let mut systems = SystemExecutor::new();

    // Register common before server code, so
    // that packet broadcasting happens after
    // gameplay actions.
    common::register(game, &mut systems);
    server.link_with_game(game, &mut systems);

    print_systems(&systems);

    game.system_executor = Rc::new(RefCell::new(systems));
}

fn init_worlds(game: &mut Game, config: &Config) {
    for world in &config.worlds.worlds {
        //let seed = 42; // FIXME: load from the level file

        game.ecs.spawn((
            WorldName::new(world.to_string()),
            WorldPath::new(PathBuf::from(format!("worlds/{}", world))),
            Dimensions::default(),
        ));
    }
}

fn init_dimensions(game: &mut Game, config: &Config) {
    let biomes = game.resources.get::<Arc<BiomeList>>().unwrap();
    for namespace in std::fs::read_dir("worldgen")
        .expect("There's no worldgen/ folder. Try removing generated/ and re-running feather")
    {
        let namespace_dir = namespace.unwrap().path();
        for file in std::fs::read_dir(namespace_dir.join("dimension")).unwrap() {
            let mut dimension_info: DimensionInfo = serde_json::from_str(
                &std::fs::read_to_string(file.as_ref().unwrap().path()).unwrap(),
            )
            .unwrap();

            dimension_info.info = serde_json::from_str(
                &std::fs::read_to_string(format!(
                    "worldgen/{}/dimension_type/{}.json",
                    dimension_info.r#type.split_once(':').unwrap().0,
                    dimension_info.r#type.split_once(':').unwrap().1
                ))
                .unwrap(),
            )
            .unwrap();

            for (_, (world_name, world_path, dimensions)) in game
                .ecs
                .query::<(&WorldName, &WorldPath, &mut Dimensions)>()
                .iter()
            {
                if !dimensions
                    .iter()
                    .any(|dim| dim.info().r#type == dimension_info.r#type)
                {
                    let generator: Arc<dyn WorldGenerator> = match &config.worlds.generator[..] {
                        "flat" => Arc::new(SuperflatWorldGenerator::new(
                            SuperflatGeneratorOptions::default(),
                        )),
                        other => {
                            log::error!("Invalid generator specified in config.toml: {}", other);
                            std::process::exit(1);
                        }
                    };
                    let is_flat = config.worlds.generator == "flat";

                    log::info!(
                        "Adding dimension `{}` to world `{}`",
                        dimension_info.r#type,
                        **world_name
                    );
                    dimensions.add(Dimension::new(
                        dimension_info.clone(),
                        generator,
                        world_path
                            .join("dimensions")
                            .join(dimension_info.r#type.split_once(':').unwrap().0)
                            .join(dimension_info.r#type.split_once(':').unwrap().1),
                        false,
                        is_flat,
                        Arc::clone(&*biomes),
                    ));
                }
            }
        }
    }
}

fn init_biomes(game: &mut Game) {
    let mut biomes = BiomeList::default();

    for dir in std::fs::read_dir("worldgen/").unwrap().flatten() {
        for file in std::fs::read_dir(dir.path().join("worldgen/biome")).unwrap() {
            if let Some(file_name) = file.as_ref().unwrap().file_name().to_str() {
                if file_name.ends_with(".json") {
                    let biome: BiomeGeneratorInfo = serde_json::from_str(
                        &std::fs::read_to_string(file.as_ref().unwrap().path()).unwrap(),
                    )
                    .unwrap();
                    let name = format!(
                        "{}:{}",
                        dir.file_name().to_str().unwrap(),
                        file_name.strip_suffix(".json").unwrap()
                    );
                    log::trace!("Loaded biome: {}", name);
                    biomes.insert(name, biome);
                }
            }
        }
    }
    game.insert_resource(Arc::new(biomes))
}

fn init_plugin_manager(game: &mut Game) -> anyhow::Result<()> {
    let mut plugin_manager = PluginManager::new();
    plugin_manager.load_dir(game, PLUGINS_DIRECTORY)?;

    let plugin_manager_rc = Rc::new(RefCell::new(plugin_manager));
    game.insert_resource(plugin_manager_rc);
    Ok(())
}

fn print_systems(systems: &SystemExecutor<Game>) {
    let systems: Vec<&str> = systems.system_names().collect();
    log::debug!("---SYSTEMS---\n{:#?}\n", systems);
}

fn run(game: Game) {
    let tick_loop = create_tick_loop(game);
    log::debug!("Launching the game loop");
    tick_loop.run();
}

fn create_tick_loop(mut game: Game) -> TickLoop {
    TickLoop::new(move || {
        let systems = Rc::clone(&game.system_executor);
        systems.borrow_mut().run(&mut game);
        game.tick_count += 1;

        false
    })
}
