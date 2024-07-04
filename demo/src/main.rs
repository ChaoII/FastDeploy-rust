use fastdeploy_rs::enum_variables::ModelFormat;
use fastdeploy_rs::model::{Classifier, DBDetector,
                           PaddleClasModel, PaddleSegModel,
                           PPOCRv3, PPYOLOE, Recognizer};
use fastdeploy_rs::runtime_option::RuntimeOption;
use fastdeploy_rs::type_bridge::Mat;
use fastdeploy_rs::visual::classify::vis_classify;
use fastdeploy_rs::visual::detection::vis_detection;
use fastdeploy_rs::visual::ocr::vis_ocr;
use fastdeploy_rs::visual::segmentation::vis_segmentation;

pub fn test_classify() {
    let model_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/inference.pdmodel";
    let param_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/inference.pdiparams";
    let config_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PaddleClasModel::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let image = Mat::imread("demo/demo_files/dog.png");
    let result = model.predict(&image);
    println!("classify result:{:?}", result);
    let vis_image = vis_classify(&image, result.unwrap(), 1, 0.5, 0.5);
    vis_image.imwrite("classify.jpg");
}


pub fn test_classify_batch() {
    let model_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/inference.pdmodel";
    let param_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/inference.pdiparams";
    let config_file = "demo/demo_files/models/PPLCNetV2_base_ssld_infer/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PaddleClasModel::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/demo_files/dog.jpg");
    let img2 = Mat::imread("demo/demo_files/dog.jpg");
    let images = &mut vec![img1, img2];
    let result = model.batch_predict(images);
    println!("detection result:{:?}", result);
}


pub fn test_detection() {
    let model_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/model.pdmodel";
    let param_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/model.pdiparams";
    let config_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PPYOLOE::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/demo_files/detection1.jpg");
    let result = model.predict(&img1);
    println!("detection result:{:?}", result);
    //let labels = vec!["sd", "s", "rrr", "ccc"];
    let vis_image = vis_detection(&img1, result.unwrap(), 0.2, 1, 0.5);
    vis_image.imwrite("detect.jpg");
}

pub fn test_detection_batch() {
    let model_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/model.pdmodel";
    let param_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/model.pdiparams";
    let config_file = "demo/demo_files/models/ppyoloe_crn_l_36e_pphuman/infer_cfg.yml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PPYOLOE::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/demo_files/detection1.jpg");
    let img2 = Mat::imread("demo/demo_files/detection1.jpg");
    let images = &mut vec![img1, img2];
    let result = model.batch_predict(images);
    println!("detection result:{:?}", result);
}

pub fn test_segmentation() {
    let model_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/model.pdmodel";
    let param_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/model.pdiparams";
    let config_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/deploy.yaml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PaddleSegModel::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let image = Mat::imread("demo/demo_files/4.jpg");
    let result = model.predict(&image);
    println!("segmentation result:{:?}", result);
    let vis_image = vis_segmentation(&image, result.unwrap(), 0.5);
    vis_image.imwrite("segmentation.jpg");
}

pub fn test_segmentation_batch() {
    let model_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/model.pdmodel";
    let param_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/model.pdiparams";
    let config_file = "demo/demo_files/models/PP_HumanSegV1_Server_with_argmax_infer/deploy.yaml";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();
    let model = PaddleSegModel::new(model_file, param_file, config_file, &runtime_option, ModelFormat::PADDLE);
    let img1 = Mat::imread("demo/demo_files/detection1.jpg");
    let img2 = Mat::imread("demo/demo_files/detection1.jpg");
    let images = &mut vec![img1, img2];
    let result = model.batch_predict(images);
    println!("segmentation result:{:?}", result);
}


pub fn test_ppocrv3() {
    let det_model_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdmodel";
    let det_param_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdiparams";
    let det_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let cls_model_file = "demo/demo_files/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdmodel";
    let cls_param_file = "demo/demo_files/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdiparams";
    let cls_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let rec_model_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdmodel";
    let rec_param_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdiparams";
    let rec_label_path = "demo/demo_files/dict.txt";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();

    let db_detector = DBDetector::new(det_model_file, det_param_file, &det_runtime_option, &ModelFormat::PADDLE);
    let classify = Classifier::new(cls_model_file, cls_param_file, &cls_runtime_option, ModelFormat::PADDLE);
    let recognizer = Recognizer::new(rec_model_file, rec_param_file, rec_label_path, &runtime_option, ModelFormat::PADDLE);
    let ppocrv3 = PPOCRv3::new(&db_detector, &classify, &recognizer);
    let image = Mat::imread("demo/demo_files/ocr2.jpg");
    let result = ppocrv3.predict(&image);
    println!("ocr results: {:#?}", result);
    let vis_image = vis_ocr(&image, result.unwrap());
    vis_image.imwrite("ocr.jpg");
}


pub fn test_ppocrv3_batch() {
    println!("{}", std::env::current_dir().unwrap().display());
    let det_model_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdmodel";
    let det_param_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_det_infer/inference.pdiparams";
    let det_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let cls_model_file = "demo/demo_files/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdmodel";
    let cls_param_file = "demo/demo_files/models/ocrv3/ch_ppocr_mobile_v2.0_cls_infer/inference.pdiparams";
    let cls_runtime_option = RuntimeOption::new();
    det_runtime_option.use_ort_backend();

    let rec_model_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdmodel";
    let rec_param_file = "demo/demo_files/models/ocrv3/ch_PP-OCRv3_rec_infer/inference.pdiparams";
    let rec_label_path = "demo/demo_files/dict.txt";
    let runtime_option = RuntimeOption::new();
    runtime_option.use_ort_backend();

    let db_detector = DBDetector::new(det_model_file, det_param_file, &det_runtime_option, &ModelFormat::PADDLE);
    let classify = Classifier::new(cls_model_file, cls_param_file, &cls_runtime_option, ModelFormat::PADDLE);
    let recognizer = Recognizer::new(rec_model_file, rec_param_file, rec_label_path, &runtime_option, ModelFormat::PADDLE);
    let ppocrv3 = PPOCRv3::new(&db_detector, &classify, &recognizer);
    let img1 = Mat::imread("demo/demo_files/ocr2.jpg");
    let img2 = Mat::imread("demo/demo_files/ocr2.jpg");
    let images = &mut vec![img1, img2];
    let result = ppocrv3.batch_predict(images);
    println!("ocr result: {:#?}", result);
}

fn main() {
    test_classify();
    //
    // test_classify_batch();

    // test_detection();

    // test_detection_batch();
    //
    // test_segmentation();
    //
    // test_segmentation_batch();
    //
    // test_ppocrv3();
    //
    // test_ppocrv3_batch();
}
