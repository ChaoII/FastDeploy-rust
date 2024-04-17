use fastdeploy_rs::enum_variables::ModelFormat;
use fastdeploy_rs::model::{PPYOLOE, Recognizer};
use fastdeploy_rs::runtime_option::RuntimeOption;
use fastdeploy_rs::type_bridge::Mat;

pub fn test_Recognizer(batch_predict: bool) {
    let model_file = "models/ch_PP-OCRv4_rec_infer/inference.pdmodel";
    let param_file = "models/ch_PP-OCRv4_rec_infer/inference.pdiparams";
    let label_path = "dict.txt";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let mut model = Recognizer::new(model_file, param_file, label_path, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/recognizer2.jpg");
    if batch_predict {
        let img2 = Mat::imread("demo/recognizer3.jpg");
        let img3 = Mat::imread("demo/recognizer1.jpg");
        let images = &mut vec![img1, img2, img3];
        let result = model.batch_predict(images);
        println!("detection result:{:#?}", result);
    } else {
        let result = model.predict(&img1);
        println!("detection result:{:#?}", result);
    }
}

pub fn test_detection() {
    let model_file = "models/ppyoloe_crn_l_36e_pphuman/model.pdmodel";
    let param_file = "models/ppyoloe_crn_l_36e_pphuman/model.pdiparams";
    let config_file = "models/ppyoloe_crn_l_36e_pphuman/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PPYOLOE::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo2.jpg");
    let img2 = Mat::imread("demo1.jpg");
    let images = &mut vec![img1, img2];
    let result = model.batch_predict(images);
    println!("detection result:{:?}", result);
    // let s = &vis_detection(img, result, 0.45, 2, 0.5);
    // Mat::imwrite(s, "result.jpg");
    // result.str();
}

fn main() {
    // test_detection();
    test_Recognizer(true);
}
