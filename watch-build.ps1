# PowerShell script to watch for .rs file changes and auto-build WASM
$watcher = New-Object System.IO.FileSystemWatcher
$watcher.Path = ".\src"
$watcher.Filter = "*.rs"
$watcher.IncludeSubdirectories = $true
$watcher.EnableRaisingEvents = $true

Write-Host "Watching for .rs file changes in src\ directory..."
Write-Host "Press Ctrl+C to stop"
Write-Host ""

$action = {
    $details = $event.SourceEventArgs
    $name = $details.Name
    $changeType = $details.ChangeType
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    
    Write-Host "[$timestamp] $changeType detected: $name" -ForegroundColor Yellow
    
    Write-Host "Building WASM module..." -ForegroundColor Cyan
    $buildResult = & wasm-pack build --target web 2>&1
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[$timestamp] Build successful!" -ForegroundColor Green
    } else {
        Write-Host "[$timestamp] Build failed!" -ForegroundColor Red
        Write-Host $buildResult
    }
    Write-Host ""
}

Register-ObjectEvent -InputObject $watcher -EventName "Changed" -Action $action | Out-Null
Register-ObjectEvent -InputObject $watcher -EventName "Created" -Action $action | Out-Null

try {
    # Keep script running
    while ($true) {
        Start-Sleep -Seconds 1
    }
} finally {
    $watcher.Dispose()
    Get-EventSubscriber | Unregister-Event
}

