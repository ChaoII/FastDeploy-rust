use fastdeploy_rs::enum_variables::ModelFormat;
use fastdeploy_rs::model::*;
use fastdeploy_rs::model::PPYOLOE;
use fastdeploy_rs::result::DetectionResult;
use fastdeploy_rs::runtime_option::RuntimeOption;
use fastdeploy_rs::type_bridge::Mat;
use fastdeploy_rs::visual::detection::vis_detection;

fn main() {
    let model_file = "ppyoloe_crn_l_36e_pphuman/model.pdmodel";
    let param_file = "ppyoloe_crn_l_36e_pphuman/model.pdiparams";
    let config_file = "ppyoloe_crn_l_36e_pphuman/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PPYOLOE::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo2.jpg");
    let img2 = Mat::imread("demo.jpg");
    let results = &mut vec![];
    let c = &mut vec![img1, img2];
    model.batch_predict(c, results);
    // let s = &vis_detection(img, result, 0.45, 4, 1.0);
    // Mat::imwrite(s, "result.jpg");
    // result.str();
}
