val mavenGroup: String by rootProject
val mavenArtifactId: String by rootProject
val version: String by rootProject

plugins {
    alias(libs.plugins.pluginyml)
    
}

bukkit {
    main = "${group}.${rootProject.name}Plugin"
    apiVersion = "1.19"
    authors = listOf("Plasmo")
    depend = listOf(
        "MCKotlin-Paper",
        "PlasmoVerseLib",
    )
}

dependencies {
    compileOnly(libs.paper)
    compileOnly(libs.plasmolib.paper)
}

tasks {

    processResources {
        filteringCharset = Charsets.UTF_8.name()
    }

    shadowJar {
        archiveBaseName.set(rootProject.name)
        archiveClassifier.set("")
        archiveAppendix.set("")
    }

    
}