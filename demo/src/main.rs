use fastdeploy_rs::enum_variables::ModelFormat;
use fastdeploy_rs::model::{Classifier, DBDetector, PPOCRv3, PPYOLOE, Recognizer};
use fastdeploy_rs::runtime_option::RuntimeOption;
use fastdeploy_rs::type_bridge::Mat;

pub fn test_Recognizer(batch_predict: bool) {
    let model_file = "demo/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdmodel";
    let param_file = "demo/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdiparams";
    let label_path = "demo/dict.txt";
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
    let model_file = "demo/models/ppyoloe_crn_l_36e_pphuman/model.pdmodel";
    let param_file = "demo/models/ppyoloe_crn_l_36e_pphuman/model.pdiparams";
    let config_file = "demo/models/ppyoloe_crn_l_36e_pphuman/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PPYOLOE::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/detection1.jpg");
    let img2 = Mat::imread("demo/detection1.jpg");
    let images = &mut vec![img1, img2];
    let result = model.batch_predict(images);
    println!("detection result:{:?}", result);
    // let s = &vis_detection(img, result, 0.45, 2, 0.5);
    // Mat::imwrite(s, "result.jpg");
    // result.str();
}

pub fn test_ppocrv3() {
    let det_model_file = "demo/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdmodel";
    let det_param_file = "demo/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdiparams";
    let det_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let cls_model_file = "demo/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdmodel";
    let cls_param_file = "demo/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdiparams";
    let cls_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let rec_model_file = "demo/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdmodel";
    let rec_param_file = "demo/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdiparams";
    let rec_label_path = "demo/dict.txt";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();

    let db_detector = DBDetector::new(det_model_file, det_param_file, &det_runtime_option, &ModelFormat::PADDLE);
    let classify = Classifier::new(cls_model_file, cls_param_file, &cls_runtime_option, ModelFormat::PADDLE);
    let recognizer = Recognizer::new(rec_model_file, rec_param_file, rec_label_path, &runtime_option, ModelFormat::PADDLE);
    let ppocrv3 = PPOCRv3::new(&db_detector, &classify, &recognizer);
    let image = Mat::imread("demo/ocr2.jpg");
    let result = ppocrv3.predict(image);
    println!("{:#?}", result);
}

fn main() {
    test_ppocrv3();
    // test_detection();
    // test_Recognizer(true);
}
