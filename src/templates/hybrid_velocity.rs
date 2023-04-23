use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/hybrid_velocity.stpl")]
pub struct HybridVelocityTemplate {
    pub maven_publish: bool,
    pub plasmo_lib: bool,
    pub run_paper: bool,
}