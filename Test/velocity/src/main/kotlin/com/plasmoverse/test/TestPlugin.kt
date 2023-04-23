import com.plasmoverse.lib.database.DatabaseConfig
import com.plasmoverse.lib.database.DatabaseFactory
import com.plasmoverse.lib.locale.VelocityLocaleLoader
import com.plasmoverse.lib.extension.velocityConfig
import com.google.inject.Inject
import com.velocitypowered.api.plugin.Plugin
import com.velocitypowered.api.plugin.annotation.DataDirectory
import com.velocitypowered.api.plugin.Dependency
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.event.Subscribe

@Plugin(
	id = "test",
	name = "Test",
	version = "1.0.0",
	dependencies = [
		Dependency(id = "mckotlin-velocity", optional = false),		
		Dependency(id = "plasmo-verse-lib", optional = false),
	],
	authors = ["Plasmo"]
)
class TestPlugin @Inject constructor(val server: ProxyServer, val logger: Logger, @DataDirectory val dataDirectory: Path) {

	private lateinit var db: Database

	private lateinit var config: Config

	@Subscribe
	fun onProxyInitialize(event: ProxyInitializeEvent) {

		logger.info("Hello World!")		

		config = Config::class.java.velocityConfig(this, dataDirectory).load()		

		db = DatabaseFactory.postgresPool(config.databaseConfig)		

		VelocityLocaleLoader(this, base = listOf("test")).load()
	}
}