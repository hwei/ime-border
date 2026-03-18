@echo off
setlocal
"%USERPROFILE%\.cargo\bin\cargo.exe" run --quiet -- %*
