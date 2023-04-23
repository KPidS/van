import com.plasmoverse.lib.database.DatabaseConfig
import com.plasmoverse.lib.database.DatabaseFactory
import com.plasmoverse.lib.locale.PaperLocaleLoader
import com.plasmoverse.lib.extension.paperConfig
import org.bukkit.plugin.java.JavaPlugin

class TestPlugin: JavaPlugin() {

	private lateinit var db: Database

	private lateinit var config: Config

	override fun onEnable() {

		logger.info("Hello World!")		

		config = Config::class.java.paperConfig(this).load()		

		db = DatabaseFactory.postgresPool(config.databaseConfig)		

		PaperLocaleLoader(this, base = listOf("test")).load()

	}

}