val mavenGroup: String by rootProject
val mavenArtifactId: String by rootProject
val version: String by rootProject

plugins {
    idea
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.kotlin.kapt)
    alias(libs.plugins.shadow)
    
}

subprojects {
    apply(plugin = "kotlin")
    apply(plugin = "kotlin-kapt")
    apply(plugin = "com.github.johnrengelman.shadow")
    

    group = mavenGroup
    version = buildVersion

    dependencies {
        compileOnly(kotlin("stdlib"))
        if (project.name != "common") api(project(":common"))
    }

    

    tasks {
        jar {
            archiveClassifier.set("dev")
        }

        shadowJar {
            archiveBaseName.set("${rootProject.name}-${project.name.capitalize()}")
            archiveClassifier.set("")
        }

        build {
            dependsOn(shadowJar)
            if (project.name != "common") {
                doLast {
                    shadowJar.get().archiveFile.get().asFile
                        .copyTo(rootProject.buildDir.resolve("libs/${shadowJar.get().archiveFile.get().asFile.name}"), true)
                }
            }
        }
    }
}

allprojects {
    mavenCentral()
mavenLocal()
maven("https://repo.plo.su")
maven("https://repo.papermc.io/repository/maven-public/")
}

tasks {
    build {
        doLast {
            jar.get().archiveFile.get().asFile.delete()
        }
    }
}