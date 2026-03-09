@echo off
:: 启用ANSI转义码支持（Win10/11必备）
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f >nul 2>&1
chcp 65001 >nul 2>&1
setlocal enabledelayedexpansion

:: 定义颜色ANSI转义码（修复注释格式问题）
set "RED=[31m"
set "GREEN=[32m"
set "YELLOW=[33m"
set "BLUE=[34m"
set "RESET=[0m"

:: 定义基础提交备注，可自行修改
set "base_commit_msg=eagle-a"

:: 【修复版】获取当前时间，兼容所有Windows系统，避免格式错乱
:: 先清理时间变量，避免残留
set "year="
set "month="
set "day="
set "hour="
set "minute="
set "second="

:: 拆分日期和时间，避免中文星期干扰
for /f "tokens=1-3 delims=/ " %%a in ("%date%") do (
    :: 适配 月/日/年 或 日/月/年 格式
    if %%a gtr 12 (
        set "day=%%a"
        set "month=%%b"
        set "year=%%c"
    ) else (
        set "month=%%a"
        set "day=%%b"
        set "year=%%c"
    )
)

:: 拆分时间（处理小时可能带前导空格的问题）
for /f "tokens=1-3 delims=:. " %%a in ("%time%") do (
    set "hour=%%a"
    set "minute=%%b"
    set "second=%%c"
)

:: 补零处理（确保月份/日期/小时为两位数）
if !month! lss 10 set "month=0!month!"
if !day! lss 10 set "day=0!day!"
if !hour! lss 10 set "hour=0!hour!"

:: 拼接最终提交备注（基础备注 + 时间）
set "commit_msg=!base_commit_msg! - !year!-!month!-!day! !hour!:!minute!:!second!"

echo ======================
echo 开始执行Git提交推送...
echo 提交备注：!commit_msg!
echo ======================
echo.

:: 进入脚本所在目录
cd /d "%~dp0"

:: 执行git add . （蓝色）
echo %BLUE%[1/3] 执行 git add .%RESET%
git add .
if !errorlevel! neq 0 (
    echo %RED%错误：git add . 执行失败！%RESET%
    pause
    exit /b 1
)
echo %GREEN%执行成功！%RESET%
echo.

:: 执行git commit （黄色）
echo %YELLOW%[2/3] 执行 git commit -m "!commit_msg!"%RESET%
git commit -m "!commit_msg!"
if !errorlevel! neq 0 (
    echo %RED%错误：git commit 执行失败（无修改/Git配置问题）！%RESET%
    pause
    exit /b 1
)
echo %GREEN%执行成功！%RESET%
echo.

:: 显示当前工作路径
echo [准备推送] 当前工作路径：
echo %cd%
echo.

:: 执行git push （红色）
echo %RED%[3/3] 执行 git push%RESET%
git push
if !errorlevel! neq 0 (
    echo %RED%错误：git push 执行失败（网络/仓库权限问题）！%RESET%
    pause
    exit /b 1
)
echo %GREEN%执行成功！%RESET%
echo.

echo ======================
echo Git 提交+推送 全部完成！
echo ======================
pause