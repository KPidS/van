use sailfish::TemplateOnce;
use crate::model::project_type::ProjectType;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/libs.stpl")]
pub struct LibsTemplate {
    pub plasmo_lib: bool,
    pub run_paper: bool,
    pub project_type: ProjectType,
}