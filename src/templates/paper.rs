use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/paper.stpl")]
pub struct PaperTemplate {
    pub maven_publish: bool,
    pub plasmo_lib: bool,
    pub run_paper: bool,
}