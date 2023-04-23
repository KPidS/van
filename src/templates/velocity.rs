use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/velocity.stpl")]
pub struct VelocityTemplate {
    pub maven_publish: bool,
    pub plasmo_lib: bool,
}