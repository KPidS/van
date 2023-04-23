use std::ffi::OsStr;
use std::fs::write;
use path_macro::path;
use crate::model::plasmo_lib_options::{PlasmoLibFeature, PlasmoLibOptions};
use crate::model::project_options::ProjectOptions;
use crate::model::project_type::{Platform, ProjectType};
use crate::utils::string::Tabulate;

pub fn scaffold_paper_main(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let class_name = format!("{}Plugin", &options.pascal_name);

    let imports = get_plasmo_lib_imports(Platform::Paper, options)
        .chain(std::iter::once(
            "org.bukkit.plugin.java.JavaPlugin".to_string(),
        ))
        .map(|path| format!("import {path}"))
        .intersperse("\n".to_string())
        .collect::<String>();

    let on_enable_content = std::iter::once("logger.info(\"Hello World!\")".to_string())
        .chain(
            get_plasmo_lib_init(Platform::Paper, options)
        )
        .intersperse("\n\n".to_string())
        .map(Tabulate::tabulate)
        .map(Tabulate::tabulate)
        .collect::<String>();

    let plugin_content = get_plasmo_lib_lateinits(options)
        .chain(std::iter::once("\toverride fun onEnable() {".to_string()))
        .chain(std::iter::once(on_enable_content))
        .chain(std::iter::once("\t}".to_string()))
        .intersperse("\n\n".to_string())
        .collect::<String>();

    let plugin = std::iter::once(format!("class {}: JavaPlugin() {{", &class_name))
        .chain(std::iter::once(plugin_content))
        .chain(std::iter::once("}".to_string()))
        .intersperse("\n\n".to_string())
        .collect::<String>();

    let contents = [imports, plugin].join("\n\n");

    let plugin_path = path!(options.code_path(module) / format!("{class_name}.kt"));

    write(plugin_path, contents)?;

    Ok(())
}

pub fn scaffold_velocity_main(module: Option<&OsStr>, options: &ProjectOptions) -> anyhow::Result<()> {

    let class_name = format!("{}Plugin", &options.pascal_name);

    let imports = get_plasmo_lib_imports(Platform::Velocity, options)
        .chain([
            "com.google.inject.Inject".to_string(),
            "com.velocitypowered.api.plugin.Plugin".to_string(),
            "com.velocitypowered.api.plugin.annotation.DataDirectory".to_string(),
            "com.velocitypowered.api.plugin.Dependency".to_string(),
            "com.velocitypowered.api.event.proxy.ProxyInitializeEvent".to_string(),
            "com.velocitypowered.api.event.Subscribe".to_string(),
        ])
        .map(|path| format!("import {path}"))
        .intersperse("\n".to_string())
        .collect::<String>();

    let plasmo_lib_dependency = options.plasmo_lib.as_ref().map(|_| String::from("plasmo-verse-lib"));

    let dependencies = std::iter::once("mckotlin-velocity".to_string())
        .chain(plasmo_lib_dependency)
        .map(|dependency| format!("Dependency(id = \"{dependency}\", optional = false),"))
        .intersperse("\n".to_string())
        .map(Tabulate::tabulate)
        .map(Tabulate::tabulate)
        .collect::<String>();

    let dependencies_parameter = [
        format!("\tdependencies = ["),
        dependencies,
        format!("\t]"),
    ].join("\n");

    let plugin_parameters = [
        format!("\tid = \"{}\"", &options.snake_name),
        format!("\tname = \"{}\"", &options.pascal_name),
        format!("\tversion = \"1.0.0\""),
        dependencies_parameter,
        format!("\tauthors = [\"Plasmo\"]"),
    ]
        .into_iter()
        .intersperse(",\n".to_string())
        // .map(Tabulate::tabulate)
        .collect::<String>();

    let class = format!(
        "class {class_name} @Inject constructor(val server: ProxyServer, val logger: Logger, @DataDirectory val dataDirectory: Path) {{"
    );

    let on_proxy_initialize = std::iter::once("logger.info(\"Hello World!\")".to_string())
        .chain(
            get_plasmo_lib_init(Platform::Velocity, options)
        )
        .intersperse("\n\n".to_string())
        .map(Tabulate::tabulate)
        .map(Tabulate::tabulate)
        .collect::<String>();

    let plugin_content = get_plasmo_lib_lateinits(options)
        .intersperse("\n\n".to_string())
        .chain(std::iter::once("\n\n".to_string()))
        .chain(std::iter::once("\t@Subscribe\n".to_string()))
        .chain(std::iter::once("\tfun onProxyInitialize(event: ProxyInitializeEvent) {\n\n".to_string()))
        .chain(std::iter::once(on_proxy_initialize))
        .chain(std::iter::once("\n\t}".to_string()))
        .collect::<String>();

    let plugin = std::iter::once("@Plugin(".to_string())
        .chain(std::iter::once("\n".to_string()))
        .chain(std::iter::once(plugin_parameters))
        .chain(std::iter::once("\n)\n".to_string()))
        .chain(std::iter::once(class))
        .chain(std::iter::once("\n\n".to_string()))
        .chain(std::iter::once(plugin_content))
        .chain(std::iter::once("\n".to_string()))
        .chain(std::iter::once("}".to_string()))
        .collect::<String>();

    let contents = [imports, plugin].join("\n\n");

    let plugin_path = path!(options.code_path(module) / format!("{class_name}.kt"));

    write(plugin_path, contents)?;

    Ok(())
}

pub fn get_plasmo_lib_imports(platform: Platform, options: &ProjectOptions) -> Box<dyn Iterator<Item = String>> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Box::new(None.into_iter());
    };

    let database_import = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => vec![
            "com.plasmoverse.lib.database.DatabaseConfig".to_string(),
            "com.plasmoverse.lib.database.DatabaseFactory".to_string(),
        ],
        false => Vec::new(),
    };

    let config_import = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Config) {
        true => match platform {
            Platform::Paper => Some("com.plasmoverse.lib.extension.paperConfig".to_string()),
            Platform::Velocity => Some("com.plasmoverse.lib.extension.velocityConfig".to_string()),
        }
        false => None,
    };

    let locale_import = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Locale) {
        true => match platform {
            Platform::Paper => Some("com.plasmoverse.lib.locale.PaperLocaleLoader".to_string()),
            Platform::Velocity => Some("com.plasmoverse.lib.locale.VelocityLocaleLoader".to_string()),
        }
        false => None,
    };

    let iter = database_import.into_iter()
        .chain(locale_import)
        .chain(config_import);

    Box::new(iter)
}

pub fn get_plasmo_lib_init(platform: Platform, options: &ProjectOptions) -> Box<dyn Iterator<Item = String>> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Box::new(None.into_iter());
    };

    let config_init = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Config) {
        true => match platform {
            Platform::Paper => Some("config = Config::class.java.paperConfig(this).load()".to_string()),
            Platform::Velocity => Some("config = Config::class.java.velocityConfig(this, dataDirectory).load()".to_string()),
        },
        false => None,
    };

    let database_init = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => Some("db = DatabaseFactory.postgresPool(config.databaseConfig)".to_string()),
        false => None,
    };

    let locale_init = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Config) {
        true => match platform {
            Platform::Paper => Some(format!("PaperLocaleLoader(this, base = listOf(\"{}\")).load()", &options.snake_name)),
            Platform::Velocity => Some(format!("VelocityLocaleLoader(this, base = listOf(\"{}\")).load()", &options.snake_name)),
        },
        false => None,
    };

    let iter = config_init
        .into_iter()
        .chain(database_init)
        .chain(locale_init);

    return Box::new(iter);
}


pub fn get_plasmo_lib_lateinits(options: &ProjectOptions) -> Box<dyn Iterator<Item = String>> {

    let Some(plasmo_lib_options) = &options.plasmo_lib else {
        return Box::new(None.into_iter());
    };

    let config_lateinit = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Config) {
        true => Some("config: Config".to_string()),
        false => None,
    };

    let database_lateinit = match plasmo_lib_options.features.contains(&PlasmoLibFeature::Database) {
        true => Some("db: Database".to_string()),
        false => None,
    };

    let iter = database_lateinit
        .into_iter()
        .chain(config_lateinit)
        .map(|string| format!("private lateinit var {string}"))
        .map(Tabulate::tabulate);

    return Box::new(iter);
}