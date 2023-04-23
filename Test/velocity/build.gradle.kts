val mavenGroup: String by rootProject
val mavenArtifactId: String by rootProject
val version: String by rootProject

dependencies {
    compileOnly(libs.velocity)
    kapt(libs.velocity)
    compileOnly(libs.plasmolib.velocity)
}

kotlin {
    jvmToolchain(17)
}

tasks.build {
    dependsOn(tasks.shadowJar.get())
}