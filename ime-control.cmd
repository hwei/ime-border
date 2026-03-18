@echo off
setlocal
set "PYTHONPATH=%~dp0cli\python"
"%~dp0.conda\python.exe" -m ime_control_main %*
exit /b %errorlevel%
