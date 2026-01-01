@echo off
REM Simple batch wrapper to run PowerShell watcher
powershell.exe -ExecutionPolicy Bypass -File "%~dp0watch-build.ps1"

