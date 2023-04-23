use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/hybrid_paper.stpl")]
pub struct HybridPaperTemplate {
    pub maven_publish: bool,
    pub plasmo_lib: bool,
    pub run_paper: bool,
}