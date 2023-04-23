use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "build_gradle_kts/hybrid_common.stpl")]
pub struct HybridCommonTemplate;