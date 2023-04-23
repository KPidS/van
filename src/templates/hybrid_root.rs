use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/hybrid_root.stpl")]
pub struct HybridRootTemplate {
    pub(crate) maven_publish: bool,
}