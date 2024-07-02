@echo off
setlocal

REM 设置Visual Studio环境变量
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat"

REM 设置路径和变量
set OUT_DIR=E:\FastDeploy-rust\target\release
set INSTALL_DIR=E:\FastDeploy\install
set NINJA_PATH=C:\software\CLion 2022.3.2\bin\ninja\win\x64\ninja.exe
set OPENCV_DIR=E:\develop\opencv4.9\build\x64\vc16\lib



REM 克隆FastDeploy仓库
if not exist "%OUT_DIR%\FastDeploy" (
    git clone https://github.com/PaddlePaddle/FastDeploy.git %OUT_DIR%\FastDeploy
    if errorlevel 1 (
        echo Git clone failed!
        exit /b 1
    )
)

REM 运行CMake配置和构建
cd %OUT_DIR%
cmake -G Ninja ^
    -DCMAKE_MAKE_PROGRAM=%NINJA_PATH% ^
    -DOPENCV_DIRECTORY=%OPENCV_DIR% ^
    -DENABLE_VISION=ON ^
    -DENABLE_ORT_BACKEND=ON ^
    -DBUILD_PADDLE2ONNX=ON ^
    -DWITH_CAPI=ON ^
    -DCMAKE_BUILD_TYPE=Release ^
    -DCMAKE_VERBOSE_MAKEFILE=ON ^
    -S %OUT_DIR%\FastDeploy ^
    -B %OUT_DIR%\build

if errorlevel 1 (
    echo CMake configuration failed!
    exit /b 1
)

REM 构建项目
cmake --build %OUT_DIR%\build --parallel 18 --verbose
if errorlevel 1 (
    echo Build failed!
    exit /b 1
)

REM 安装项目
cmake --install %OUT_DIR%\build --prefix %INSTALL_DIR%
if errorlevel 1 (
    echo Installation failed!
    exit /b 1
)

echo Build and installation completed successfully!

endlocal
exit /b 0
