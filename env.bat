@echo off
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo 请以管理员身份运行此脚本!
    pause
    exit /b
)

setx PATH "%PATH%;C:\Program Files (x86)\VideoLAN\VLC" /M
echo 已将 C:\Program Files (x86)\VideoLAN\VLC 添加到系统环境变量 PATH
pause